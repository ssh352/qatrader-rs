pub mod qamongo;
pub mod qawebsocket;
pub mod qaeventmq;
extern crate ndarray;

extern crate chrono;
use chrono::prelude::*;
use ndarray::array;

use std::time::Duration;
use std::thread;
use qaeventmq::Subscriber;
use crate::qaeventmq::Publisher;

fn main() {

    qamongo::query_account("192.168.2.24".to_string(), "288870".to_string());
    let mut puber = qaeventmq::QAEventMQ{
        amqp: "amqp://admin:admin@192.168.2.118:5672/".to_string(),
        exchange: "tick".to_string(),
        model: "direct".to_string(),
        routing_key: "rb2001".to_string()
    };
    let mut i = 1;
    thread::spawn(move|| {
        while i<1000 {
            puber.publish_routing("s".to_string());
            i+=1;
            thread::sleep(Duration::from_secs(5));
        }


    });

    let mut client = qaeventmq::QAEventMQ{
        amqp: "amqp://admin:admin@192.168.2.118:5672/".to_string(),
        exchange: "tick".to_string(),
        model: "direct".to_string(),
        routing_key: "rb2001".to_string()
    };

    thread::spawn(move || {
        client.subscribe_routing();

    });


     qawebsocket::wsmain(
         "ws://101.132.37.31:7988".to_string());

    test_ndarray();
    test_datetime();
    test_timeseries();

}


fn test_ndarray() {
    let a3 = array![[[1, 2], [3, 4]],
                    [[5, 6], [7, 8]]];
    println!("{}", a3);

}


pub struct Quote {
    pub datetime: String,
    pub code: String,
    pub open: i32,
    pub high: i32,
    pub low: i32,
    pub close: i32,
}

impl Quote {
    pub fn new(code: &str, datetime: &str, open: i32, high: i32, low: i32, close: i32) -> Quote {
        Quote {
            code: code.to_string(),
            datetime: datetime.to_string(),
            open,
            high,
            low,
            close,
        }
    }

    pub fn update(&mut self) {


        let dt: chrono::DateTime<Utc> = chrono::Utc::now();
        let fixed_dt = dt.with_timezone(&FixedOffset::east(8*3600));
        let data = array![4392, 4435, 4285, 9999999];
        println!("{}", data[0]);
        fixed_dt.to_string();
        "rb2001".to_string();
    }
}






fn test_datetime() {
    let dt: chrono::DateTime<Utc> = chrono::Utc::now();
    let fixed_dt = dt.with_timezone(&FixedOffset::east(8*3600));
    println!("{}", dt);
    println!("{}", fixed_dt);
}


// fn test_pyo3() -> Result<(), ()> {
//     let gil = Python::acquire_gil();
//     test_pyo3_(gil.python()).map_err(|e| {
//         eprintln!("error! :{:?}", e);
//         // we can't display python error type via ::std::fmt::Display
//         // so print error here manually
//         e.print_and_set_sys_last_vars(gil.python());
//     })
// }

// fn test_pyo3_<'py>(py: Python<'py>) -> PyResult<()> {
//     let np = py.import("numpy")?;
//     let dict = PyDict::new(py);
//     dict.set_item("np", np)?;
//     let pyarray: &PyArray1<i32> = py
//         .eval("np.absolute(np.array([-1, -2, -3], dtype='int32'))", Some(&dict), None)?
//         .extract()?;
//     let slice = pyarray.as_slice()?;
//     assert_eq!(slice, &[1, 2, 3]);
//     Ok(())
// }




fn test_timeseries() {
    let mut stock = Quote::new("rb2001", "2019", 1, 2, 3, 4);
    println!("Current OPEN: {}", stock.open);
    stock.update();
}