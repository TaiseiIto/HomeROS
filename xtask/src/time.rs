use crate::command::get_stdout;

pub fn zone() -> String {
    get_stdout("readlink /etc/localtime")
}
