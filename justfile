#!/usr/bin/env just --justfile

# create a new AoC day using template, eg. $ just start 2023 04
create year day:
    cp -r .template {{year}}/day-{{day}}
    code {{year}}/day-{{day}}

# use TUI to create AoC day from template
start:
    #!/usr/bin/env cargo +nightly -Zscript

    //! ```cargo
    //! [package]
    //! edition = "2021"
    //!
    //! [dependencies]
    //! inquire = { version = "0.6.2", features = ["date"] }
    //! chrono = "0.4.31"
    //! reqwest = { version = "0.11.22", features=["blocking"] }
    //! ```

    use inquire::{error::InquireResult, DateSelect, Select};
    use chrono::NaiveDate;
    use chrono::prelude::*;
    use std::process::Command;

    fn main() -> InquireResult<()> {

        // TODO: other editions are not supported, as `DateSelect` will fait and needs to be changed with `with_starting_date`

        let editions = vec! [2023]; // vec![2015, 2016, 2017, 2018, 2019, 2020, 2021, 2022, 2023];

        let num_editions = editions.len();

        let edition = Select::new("Choose and AoC event", editions)
            .with_starting_cursor(num_editions-1)
            .prompt()?;

        println!("{}",edition);


        let date = DateSelect::new("Date:")
            .with_min_date(NaiveDate::from_ymd_opt(edition, 12, 1).unwrap())
            .with_max_date(NaiveDate::from_ymd_opt(edition, 12, 25).unwrap())
            .prompt()?;

        let day = date.day();

        let year = date.year();


        let status = Command::new("just")
            .arg("create")
            .arg(year.to_string())
            .arg(day.to_string())
            .status()
            .expect("failed to execute process");

        println!("process finished with: {status}");

        assert!(status.success());

        Ok(())
    }
