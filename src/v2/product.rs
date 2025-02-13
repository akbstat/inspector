use super::{
    error::Result,
    trial::{list_trials, Trial},
};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use std::{fs, path::Path};

lazy_static! {
    static ref PRODUCT_NAME: Regex = Regex::new(r"^ak\d{3}").unwrap();
}

#[derive(Debug, Serialize)]
pub struct Product {
    id: String,
    name: String,
    trials: Vec<Trial>,
}

pub fn list_products<P: AsRef<Path>>(root: P) -> Result<Vec<Product>> {
    let mut products = vec![];
    for entry in fs::read_dir(&root)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            continue;
        }
        let dir = entry.path();
        if let Some(product) = fetch_product(&dir)? {
            products.push(product);
        }
    }
    Ok(products)
}

fn fetch_product<P: AsRef<Path>>(root: P) -> Result<Option<Product>> {
    let product = root
        .as_ref()
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_lowercase();
    if !valid_product_name(&product) {
        Ok(None)
    } else {
        Ok(Some(Product {
            id: product.clone(),
            name: product.clone(),
            trials: list_trials(&product, root)?,
        }))
    }
}

fn valid_product_name(name: &str) -> bool {
    PRODUCT_NAME.is_match(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_list_products() -> anyhow::Result<()> {
        let root = Path::new(r"D:\Studies");
        let products = list_products(root)?;
        assert_eq!(products.len(), 10);
        Ok(())
    }

    #[test]
    fn test_fetch_product() -> anyhow::Result<()> {
        let root = Path::new(r"D:\Studies\ak101");
        let product = fetch_product(root)?;
        assert!(product.is_some());
        Ok(())
    }

    #[test]
    fn test_valid_product_name() {
        let name = "ak101";
        assert!(valid_product_name(name));
        let name = "bk101";
        assert!(!valid_product_name(name));
        let name = "";
        assert!(!valid_product_name(name));
    }
}
