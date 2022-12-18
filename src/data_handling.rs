/*- Imports -*/
use std::fs::{ self, File };
use std::io::{BufRead, BufReader};
use std::io::{Cursor, Read, Seek};
use crate::network::DataPoint;
use serde_derive::Deserialize;
use image;

/*- Constants -*/
const PATH_TO_LABELS:&'static str = "./mnist/labels";
const PATH_TO_IMAGES:&'static str = "./mnist/images";
const PATH_TO_JSON_DATA: &'static str = "./learning_data.json";

/*- Load training data -*/
#[derive(Deserialize)]
struct JsonLoad { learning_data:Vec<Vec<Vec<f64>>> }
pub fn load_json_data() -> Vec<DataPoint> {
    let file = fs::read_to_string(PATH_TO_JSON_DATA).expect("Unable to read file");
    let json:JsonLoad = serde_json::from_str(&file).expect("Unable to parse json");
    let mut datapoints:Vec<DataPoint> = Vec::with_capacity(json.learning_data.len());

    /*- Iterate over all data points -*/
    for data_point in json.learning_data {
        let data = &data_point[0];
        let labels = &data_point[1];

        /*- Create datapoint -*/
        let dp = DataPoint { labels: labels.clone(), data: data.clone() };
        datapoints.push(dp);
    };

    datapoints
}


/*
    Load mnist images into a vector of vectors of u8.
    The vectors of u8 contain the pixel values of the images.
    The file is structured as follows:
    [offset] [type]          [value]          [description]
    0000     32 bit integer  0x00000803(2051) magic number
    0004     32 bit integer  60000            number of images
    0008     32 bit integer  28               number of rows
    0012     32 bit integer  28               number of columns
    0016     unsigned byte   ??               pixel
    0017     unsigned byte   ??               pixel
    ........
    xxxx     unsigned byte   ??               pixel
    Pixels are organized row-wise. Pixel values are 0 to 255. 0 means background (white), 255 means foreground (black).
*/
pub fn load_mnist_data() -> Result<Vec<Vec<f64>>, String> {
    println!("Loading mnist data...");
    let mut images:Vec<Vec<f64>> = Vec::new();

    /*- Open file -*/
    let file = File::open(PATH_TO_IMAGES).expect("Unable to open file");
    let mut reader = BufReader::new(file);

    /*- Read magic number -*/
    let mut magic_number = [0; 4];
    reader.read_exact(&mut magic_number).expect("Unable to read magic number");
    if magic_number != [0, 0, 8, 3] {
        return Err("Magic number is not 0x00000803".to_string());
    }

    /*- Read number of images -*/
    let mut number_of_images = [0; 4];
    reader.read_exact(&mut number_of_images).expect("Unable to read number of images");
    let number_of_images = u32::from_be_bytes(number_of_images);

    /*- Read number of rows -*/
    let mut number_of_rows = [0; 4];
    reader.read_exact(&mut number_of_rows).expect("Unable to read number of rows");
    let number_of_rows = u32::from_be_bytes(number_of_rows);

    /*- Read number of columns -*/
    let mut number_of_columns = [0; 4];
    reader.read_exact(&mut number_of_columns).expect("Unable to read number of columns");
    let number_of_columns = u32::from_be_bytes(number_of_columns);

    /*- Read images -*/
    for _ in 0..number_of_images {
        let mut image:Vec<f64> = Vec::with_capacity((number_of_rows * number_of_columns) as usize);
        for _ in 0..(number_of_rows * number_of_columns) {
            let mut pixel = [0; 1];
            reader.read_exact(&mut pixel).expect("Unable to read pixel");
            image.push(pixel[0] as f64);
        }
        images.push(image);
    }

    Ok(images)
}
pub fn load_mnist_labels() -> Result<Vec<Vec<f64>>, String> {
    println!("Loading mnist labels...");
    let mut labels:Vec<f64> = Vec::new();

    /*- Open file -*/
    let file = File::open(PATH_TO_LABELS).expect("Unable to open file");
    let mut reader = BufReader::new(file);

    /*- Read magic number -*/
    let mut magic_number = [0; 4];
    reader.read_exact(&mut magic_number).expect("Unable to read magic number");
    if magic_number != [0, 0, 8, 1] {
        return Err("Magic number is not 0x00000801".to_string());
    }

    /*- Read number of labels -*/
    let mut number_of_labels = [0; 4];
    reader.read_exact(&mut number_of_labels).expect("Unable to read number of labels");
    let number_of_labels = u32::from_be_bytes(number_of_labels);

    /*- Read labels -*/
    for _ in 0..number_of_labels {
        let mut label = [0; 1];
        reader.read_exact(&mut label).expect("Unable to read label");
        labels.push(label[0] as f64);
    }

    /*- Convert labels to one-hot vectors -*/
    let mut one_hot_labels:Vec<Vec<f64>> = Vec::with_capacity(labels.len());
    for label in labels {
        let mut one_hot_label:Vec<f64> = Vec::with_capacity(10);
        for i in 0..10 {
            if i == label as usize {
                one_hot_label.push(1f64);
            } else {
                one_hot_label.push(0f64);
            }
        }
        one_hot_labels.push(one_hot_label);
    }

    Ok(one_hot_labels)
}


pub fn mnist_image_to_png(input:Vec<u8>) -> () {
    let mut image = image::ImageBuffer::new(28, 28);
    let mut i = 0;
    for (_, _, pixel) in image.enumerate_pixels_mut() {
        let value = input[i];
        *pixel = image::Rgb([value, value, value]);
        i += 1;
    }
    image.save("test.png").unwrap();
}