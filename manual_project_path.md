# project name

## init

touch README.md \
&& ln -s README.md README \
&& cargo init "." \
&& cargo add rustfmt \
&& rustup component add rustfmt \
&& mkdir examples \
&& cp src/main.rs examples/example.rs \
&& sed -i -e 's/world/example/g' examples/example.rs \
&& rustup  show \
&& rustup  check \
&& rustup toolchain uninstall stable \
&& rustup toolchain install stable \
&& rustup update  --force \
&& rustup show \
&& mkdir tests

## How to launch a Rust application from Visual Studio Code? [![alt text][1]](https://stackoverflow.com/questions/46885292/how-to-launch-a-rust-application-from-visual-studio-code)

## prompt

- Create a new file with name project_path.md and add header project name , project create, os version
- Pls add the date of create file with timezone
- Pls run the program on the console and fix the error until you have no error anymore. For each rerun ask for next procedure.
  
I never plan never far ahead. Carpe diam

<!-- Link sign - Don't Found a better way :-( - You know a better method? - send me a email -->
[1]: ./img/link_symbol.svg
