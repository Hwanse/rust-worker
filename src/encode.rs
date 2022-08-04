use rand::Rng;
use std::process::Command;

const INPUT_DIRECTORY: &str = "samples";
const OUTPUT_DIRECTORY: &str = "output";
const EXEC_SHELL: &str = "/bin/sh";
const M4A: &str = ".m4a";

pub fn acc_encode(input_name: &str) {
    let input_file = format!("{}/{}", INPUT_DIRECTORY, input_name);

    let random_number: u32 = rand::thread_rng().gen();
    let output_name = format!("output_{}", random_number);
    let output_file = format!(
        "{path}/{name}{extension}",
        path = OUTPUT_DIRECTORY,
        name = &output_name,
        extension = M4A
    );

    let child = Command::new(EXEC_SHELL)
        .args(&[
            "-c",
            format!(
                "ffmpeg -i {input} -c:a aac -b:a 160k {output}",
                input = input_file.as_str(),
                output = output_file.as_str()
            )
            .as_str(),
        ])
        .output()
        .expect("failed to execute ffmpeg");

    println!("whether ffmpeg work success : {}", child.status.success());
}
