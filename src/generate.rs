use rand::prelude::SliceRandom;
use std::{
    fs,
    io::{self},
};

use crate::file_utils;
use rand::Rng;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Generate {
    days: Vec<String>,
    pool: Vec<String>,
}

#[derive(Eq, PartialEq)]
pub enum Collection {
    Days,
    Pool,
}

#[allow(dead_code)]
impl Generate {
    fn new(days: Vec<String>, pool: Vec<String>) -> Self {
        Self { days, pool }
    }

    pub fn days(&self) -> &Vec<String> {
        &self.days
    }

    pub fn pool(&self) -> &Vec<String> {
        &self.pool
    }

    pub fn read_entries(
        inputfile: &str,
        outputfile: &str,
        mut days: usize,
        reset: bool,
    ) -> Result<Self, io::Error> {
        let pool = Self::prepare_pool(inputfile, outputfile, reset)?;
        if reset {
            Self::reset_output_file(outputfile)?;
        }
        if let Ok(entries) = file_utils::read_file(outputfile) {
            Ok(Self {
                days: entries,
                pool,
            })
        } else {
            Ok(Self::new(vec![], pool).generate_days(&mut days))
        }
    }

    pub fn generate_days(&mut self, days: &mut usize) -> Self {
        if *days > self.pool.len() {
            *days = self.pool.len()
        }

        let selected_entries = Self::select_random_entries(&mut self.pool, *days);
        Self {
            days: selected_entries,
            pool: self.pool.clone(),
        }
    }

    fn prepare_pool(
        inputfile: &str,
        outputfile: &str,
        reset: bool,
    ) -> Result<Vec<String>, io::Error> {
        if !std::path::Path::new(outputfile).exists() || reset {
            println!("outputfile no exists");
            file_utils::read_file(inputfile)
        } else {
            println!("{} outputfile exists", outputfile);
            let initial = file_utils::read_file(inputfile)?;
            let prev = file_utils::read_file(outputfile)?;
            Ok(Self::subtract_slices(initial, prev))
        }
    }

    fn reset_output_file(outputfile: &str) -> Result<(), io::Error> {
        if std::path::Path::new(outputfile).exists() {
            println!("Resetting output");
            let _ = fs::remove_file::<_>(outputfile);
        }
        Ok(())
    }

    // 4. Selects random entries from the pool
    fn select_random_entries(pool: &mut Vec<String>, days: usize) -> Vec<String> {
        let mut selected_entries = Vec::with_capacity(days);
        let mut rng = rand::thread_rng();

        while selected_entries.len() < days {
            let index = rng.gen_range(0..pool.len());
            selected_entries.push(pool.remove(index));
        }

        selected_entries
    }

    fn subtract_slices(slice1: Vec<String>, slice2: Vec<String>) -> Vec<String> {
        slice1.into_iter().filter(|v| !slice2.contains(v)).collect()
    }

    pub fn write_file(&self, file_name: &str) -> Result<(), io::Error> {
        file_utils::write_file(&self.days, file_name)
    }

    pub fn print_output(&self) {
        for (idx, entry) in self.days.iter().enumerate() {
            println!("{}: {}", idx, entry);
        }
    }

    pub fn regenerate_entry(&mut self, index: usize) -> Result<(), &'static str> {
        if self.days.is_empty() || self.pool.is_empty() {
            return Err("No entries to regenerate");
        }

        if index >= self.days.len() {
            return Err("Invalid index");
        }

        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..self.pool.len());
        let new_entry = self.pool.remove(random_index);

        let removed_entry = std::mem::replace(&mut self.days[index], new_entry);

        self.pool.push(removed_entry);

        Ok(())
    }

    pub fn find_entry(&self, col: Collection, entry: &str) -> Option<usize> {
        let entries = if col == Collection::Pool {
            &self.pool
        } else {
            &self.days
        };
        entries.iter().position(|x| x == entry)
    }

    pub fn add_to_pool(&mut self, new_entry: String) {
        self.pool.push(new_entry);
    }

    pub fn remove_from_pool_by_strvalue(
        &mut self,
        entry: &str,
        input_file: &str,
    ) -> Result<(), io::Error> {
        if let Some(pos) = self.find_entry(Collection::Pool, entry) {
            self.pool.remove(pos);
            file_utils::comment_out_in_file(input_file, entry)?;
        }
        Ok(())
    }

    pub fn remove_from_pool(&mut self, index: usize, input_file: &str) -> Result<(), io::Error> {
        self.pool.remove(index);
        file_utils::comment_out_in_file(input_file, &self.pool[index])?;
        Ok(())
    }

    pub fn edit_pool_entry_by_strvalue(
        &mut self,
        old_entry: &str,
        new_entry: String,
    ) -> Result<(), &'static str> {
        if let Some(pos) = self.find_entry(Collection::Pool, old_entry) {
            self.pool[pos] = new_entry;
            Ok(())
        } else {
            Err("Entry not found in pool")
        }
    }

    pub fn edit_pool_entry(&mut self, index: usize, new_entry: String) -> Result<(), &'static str> {
        if index >= self.pool.len() {
            return Err("Invalid index");
        }
        self.pool[index] = new_entry;
        Ok(())
    }

    pub fn edit_days_entry(&mut self, index: usize, new_entry: String) -> Result<(), &'static str> {
        if index >= self.days.len() {
            return Err("Invalid index");
        }
        self.days[index] = new_entry;
        Ok(())
    }

    pub fn swap_days_entries(&mut self, index1: usize, index2: usize) -> Result<(), &'static str> {
        if index1 >= self.days.len() || index2 >= self.days.len() {
            return Err("Invalid index");
        }
        self.days.swap(index1, index2);
        Ok(())
    }

    pub fn randomize_days(&mut self) {
        let mut rng = rand::thread_rng();
        self.days.shuffle(&mut rng);
    }
}
