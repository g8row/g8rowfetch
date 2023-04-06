use std::collections::HashMap;
use std::fmt;
use colored::Colorize;
use systemstat::{saturating_sub_bytes, Platform, System};
use wmi::Variant;
use wmi::*;

fn get_cpu_temp(com: COMLibrary) -> f32 {
    let mut temp_cel = 0.0;
    let wmi_con = WMIConnection::new(com).unwrap();

    let results: Vec<HashMap<String, Variant>> = wmi_con
        //.raw_query("SELECT * FROM Win32_TemperatureProbe")
        .raw_query("SELECT * FROM Win32_PerfFormattedData_Counters_ThermalZoneInformation")
        .unwrap();
    for os in results {
        if os.contains_key("HighPrecisionTemperature") {
            let temp = os.get("HighPrecisionTemperature").unwrap();
            match temp {
                Variant::UI4(c) => temp_cel = (c.to_owned() as f32) / 10.0 - 273.15,
                _ => temp_cel = 0.0,
            }
        }
    }
    temp_cel
}

fn get_name(com: COMLibrary) -> String {
    let mut name = String::new();
    let wmi_con = WMIConnection::new(com).unwrap();

    let results: Vec<HashMap<String, Variant>> = wmi_con
        .raw_query("SELECT * FROM Win32_ComputerSystem")
        .unwrap();
    for os in results {
        if os.contains_key("UserName") {
            let temp = os.get("UserName").unwrap();
            match temp {
                Variant::String(c) => name = c.to_owned(),
                _ => name = String::new(),
            }
        }
    }
    name
}

fn main() {
    let com = COMLibrary::new().unwrap();
    let sys = System::new();

    let mut memory = String::new();
    match sys.memory() {
        Ok(mem) => fmt::write(
            &mut memory,
            format_args!(
                "Memory: {} used / {} ({} bytes) total",
                saturating_sub_bytes(mem.total, mem.free),
                mem.total,
                mem.total.as_u64(),
            ),
        )
        .unwrap(),
        Err(x) => println!("\nMemory: error: {}", x),
    }

    /*let wmi_con = WMIConnection::new(com).unwrap();

     let results: Vec<HashMap<String, Variant>> = wmi_con
        //.raw_query("SELECT * FROM Win32_TemperatureProbe")
        .raw_query("SELECT * FROM Win32_ComputerSystem")
        .unwrap();
    for os in results{
        print!("{:?}", os)
    } */

    println!(
"       ~@@@#:       {}
  7?^. ~@@@&: .~?!  --------------------
.5@@@#5Y@@@&J5#@@@J CPU Temp : {:.2} °C 🔥
.7P#@@@@@@@@@@@&B5! {}
   :?&@@@@@@@@#7.   
.?P#@@@@@@@@@@@@#57 
.Y@@@BYJ@@@&?5#@@@? 
  !7^. ~@@@&: .~?~  
       ~&@@#:       ",
        get_name(com).blue(),
        get_cpu_temp(com),
        memory
    );
}
