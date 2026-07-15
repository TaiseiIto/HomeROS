use crate::command;

pub fn unix() -> String {
    command::get_stdout("date +%s")
}

pub fn zone() -> String {
    command::get_stdout("readlink -f /etc/localtime")
}
