use crate::command;

pub fn zone() -> String {
    command::get_stdout("readlink -f /etc/localtime")
}
