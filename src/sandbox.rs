use std::process::{Command, Stdio};
use std::{env, io};

macro_rules! docker_cmd {
    ($($arg:expr),* $(,)?) => ({
        let mut cmd = Command::new("docker");
        $( cmd.arg($arg); )*
        cmd
    });
}

fn run_cmd(mut cmd: Command) {
    let result = cmd
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to run command");

    let result_out = String::from_utf8(result.stdout)
        .expect("failed to convert to utf-8");
    if result_out != "" {
        println!("{result_out}");
    }

    let result_err = String::from_utf8(result.stderr)
        .expect("failed to convert to utf-8");
    if result_err != "" {
        eprintln!("{result_err}");
        panic!("{result_err}")
    }
}

fn build(path: &str, tag: &str) {
    let cmd = docker_cmd!(
        "build",
        path,
        "-t",
        tag
    );
    run_cmd(cmd);
}

pub fn build_compiler() {
    build("src/docker/compiler/", "accela_compiler")
}

pub fn build_core() {
    build("src/docker/core/", "accela_core");
}

pub fn build_module_base() {
    build("src/docker/module/", "accela_module");
}

fn compile(src_path: &str, name: &str, bin: &str) {
    let cmd = docker_cmd!(
        "run",
        "--rm",
        //"--runtime=runsc",
        "--mount",
        format!("type=bind,source={},target=/app/src", src_path),
        "--env",
        format!("TARGET_PKG={}", name),
        "--env",
        format!("TARGET_BIN={}", bin),
        "accela_compiler"
    );
    run_cmd(cmd);
}

fn create_module_container(src_path: &str,
                           name: &str,
                           storage: &str,
                           memory: &str,
                           memory_swap: &str,
                           pids: &str,
                           networks: Vec<&str>,
                           expose_ports: Vec<&str>,
                           publish_ports: Vec<&str>) {
    println!("Removing old version ...");
    run_cmd(docker_cmd!(
        "rm",
        "-f",
        format!("accela_module_{}", name)
    ));

    println!("Creating new version ...");
    let mut cmd = docker_cmd!(
        "create",
        "--platform",
        "linux/amd64",
        //"--runtime=runsc",
        "--cap-drop=ALL",
        // Needed to allow overwriting the file
        "--cap-add=DAC_OVERRIDE",
        "--security-opt=no-new-privileges",
        "--workdir",
        "/app",
        "--memory",
        memory,
        "--memory-swap",
        memory_swap,
        "--pids-limit",
        pids,
        "--name",
        format!("accela_module_{}", name),
    );
    if storage != "" {
        cmd.args(&["--storage-opt", &*format!("size={}", storage)]);
    }
    if !networks.is_empty() {
        for item in networks {
            cmd.args(&["--network", item]);
        }
    }
    if !expose_ports.is_empty() {
        for item in expose_ports {
            cmd.args(&["--expose", item]);
        }
    }
    if !publish_ports.is_empty() {
        for item in publish_ports {
            cmd.args(&["--publish", item]);
        }
    }
    cmd.args(&["accela_module"]);
    run_cmd(cmd);

    println!("\nCompiling ...");
    compile(src_path, name, name);

    println!("\nInjecting binary into container ...");
    run_cmd(docker_cmd!(
        "cp",
        format!("{}/target/release/{}", src_path, name),
        format!("accela_module_{}:/app/bin/main", name),
    ));
}

pub struct Sandbox {
}

impl Sandbox {
    pub fn new() -> Sandbox {
        build_compiler();
        build_module_base();
        Sandbox {}
    }

    pub fn run(self, name: &str) {
        println!("\nBuilding module container ...");

        let compile_path: &str =
            &*format!("{}/{}",
                      env::current_dir().expect("failed to get cwd").into_os_string().into_string().unwrap(),
                      name).to_owned();

        create_module_container(compile_path,
                                name,
                                "",
                                "512m",
                                "640m",
                                "32",
                                vec!("host"),
                                vec!("3301", "23"),
                                vec!("3301:3301", "23:23")
        );

        println!("\nBooting module container ...");
        //TODO run and note the ownership + container name
        run_cmd(docker_cmd!(
            "start",
            "--attach",
            format!("accela_module_{}", name)
        ));
    }
}
