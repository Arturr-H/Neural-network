## This is a program that generates learning data for the neural network I built in rust

# Imports
import random;

# Generate 100 arrays, containing firstly an array of 5 random numbers
# between 0 and 1, and secondly an array of 2 numbers, [1, 0] meaning
# that the first number in the first array is 1, and [0, 1] meaning
# that the fist number in the first array is 0
def generate_learning_data():
    learning_data = [];
    for i in range(10000):
        first_array = [];
        for j in range(10):
            first_array.append(1 if random.random() > 0.5 else 0);
        second_array = [1, 0] if first_array[0] > 0.5 else [0, 1];
        learning_data.append([first_array, second_array]);
    return learning_data;

# Write the learning data to a json file
def write_learning_data_to_file(learning_data):
    with open("learning_data.json", "w") as file:

        string = "{\"learning_data\": " + str(learning_data) + "}";
        file.write(str(string));

# Main function
def main():
    learning_data = generate_learning_data();
    write_learning_data_to_file(learning_data);

# Run main function
if __name__ == "__main__":
    main();