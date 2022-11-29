use std::env; // to get arugments passed to the program
use std::thread;

/*
* Print the number of partitions and the size of each partition
* @param vs A vector of vectors
*/
fn print_partition_info(vs: &Vec<Vec<usize>>){
    println!("Number of partitions = {}", vs.len());
    for i in 0..vs.len(){
        println!("\tsize of partition {} = {}", i, vs[i].len());
    }
}

/*
* Create a vector with integers from 0 to num_elements -1
* @param num_elements How many integers to generate
* @return A vector with integers from 0 to (num_elements - 1)
*/
fn generate_data(num_elements: usize) -> Vec<usize>{
    let mut v : Vec<usize> = Vec::new();
    for i in 0..num_elements {
        v.push(i);
    }
    return v;
}

/*
* Partition the data in the vector v into 2 vectors
* @param v Vector of integers
* @return A vector that contains 2 vectors of integers

*/
fn partition_data_in_two(v: &Vec<usize>) -> Vec<Vec<usize>>{
    let partition_size = v.len() / 2;
    // Create a vector that will contain vectors of integers
    let mut xs: Vec<Vec<usize>> = Vec::new();

    // Create the first vector of integers
    let mut x1 : Vec<usize> = Vec::new();
    // Add the first half of the integers in the input vector to x1
    for i in 0..partition_size{
        x1.push(v[i]);
    }
    // Add x1 to the vector that will be returned by this function
    xs.push(x1);

    // Create the second vector of integers
    let mut x2 : Vec<usize> = Vec::new();
    // Add the second half of the integers in the input vector to x2
    for i in partition_size..v.len(){
        x2.push(v[i]);
    }
    // Add x2 to the vector that will be returned by this function
    xs.push(x2);
    // Return the result vector
    xs
}

/*
* Sum up the all the integers in the given vector
* @param v Vector of integers
* @return Sum of integers in v
* Note: this function has the same code as the reduce_data function.
*       But don't change the code of map_data or reduce_data.
*/
fn map_data(v: &Vec<usize>) -> usize{
    let mut sum = 0;
    for i in v{
        sum += i;
    }
    sum
}

/*
* Sum up the all the integers in the given vector
* @param v Vector of integers
* @return Sum of integers in v
*/
fn reduce_data(v: &Vec<usize>) -> usize{
    let mut sum = 0;
    for i in v{
        sum += i;
    }
    sum
}

/*
* A single threaded map-reduce program
*/
fn main() {

    // Use std::env to get arguments passed to the program
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("ERROR: Usage {} num_partitions num_elements", args[0]);
        return;
    }
    let num_partitions : usize = args[1].parse().unwrap();
    let num_elements : usize = args[2].parse().unwrap();
    if num_partitions < 1{
      println!("ERROR: num_partitions must be at least 1");
        return;
    }
    if num_elements < num_partitions{
        println!("ERROR: num_elements cannot be smaller than num_partitions");
        return;
    }

    // Generate data.
    let v = generate_data(num_elements);

    // PARTITION STEP: partition the data into 2 partitions
    let xs = partition_data_in_two(&v);

    // Print info about the partitions
    print_partition_info(&xs);

    let mut intermediate_sums : Vec<usize> = Vec::new();

    // MAP STEP: Process each partition

    // CHANGE CODE START: Don't change any code above this line

    // Change the following code to create 2 threads that run concurrently and each of which uses map_data() function to process one of the two partitions

    //let p1 = &xs[0];
    //let p2 = &xs[1];

    /*Create new vectors to hold the data*/
    let mut p1 : Vec<usize> = Vec::new();
    let mut p2 : Vec<usize> = Vec::new();

    /*Iterate over the old data and push it into the new vectors*/
    for val in &xs[0]{

        p1.push(*val)

    }

    for val in  &xs[1]{

        p2.push(*val);
    }

    /*Allow threads to take ownership of new data*/
    let t1 = thread::spawn(move || map_data(&p1));
    let t2 = thread::spawn(move || map_data(&p2));


    /*Wait for the threads to complete by joining and unwrapping them*/
    let r1 = t1.join().unwrap();
    let r2 = t2.join().unwrap();

    /*Push finished data onto intermediate sums vector*/
    intermediate_sums.push(r1);
    intermediate_sums.push(r2);

    // CHANGE CODE END: Don't change any code below this line until the next CHANGE CODE comment

    // Print the vector with the intermediate sums
    println!("Intermediate sums = {:?}", intermediate_sums);

    // REDUCE STEP: Process the intermediate result to produce the final result
    let sum = reduce_data(&intermediate_sums);
    println!("Sum = {}", sum);

    // CHANGE CODE: Add code that does the following:
    // 1. Calls partition_data to partition the data into equal partitions
    // 2. Calls print_partition_info to print info on the partitions that have been created
    // 3. Creates one thread per partition and uses each thread to concurrently process one partition
    // 4. Collects the intermediate sums from all the threads
    // 5. Prints information about the intermediate sums
    // 5. Calls reduce_data to process the intermediate sums
    // 6. Prints the final sum computed by reduce_data

    /*1*/
    let ms = partition_data(num_partitions, &v);

    /*2*/
    print_partition_info(&ms);

    /*3*/

    /*Create vector to hold threads*/

    let mut thread_vector : Vec<thread::JoinHandle<usize>> = Vec::new();

    for val in ms
    {/*Iterating over our partitions vector*/

        thread_vector.push(thread::spawn(move || map_data(&val)));

    }

    /*4*/

    let mut intermediate_sums_mt : Vec<usize> = Vec::new();
    let  _thread_result_vector : Vec<usize> = Vec::new();

    for thread in thread_vector
    {
        let mt_res = thread.join().unwrap();
        intermediate_sums_mt.push(mt_res);
        
    }

    /*5*/
    println!("Intermediate sums = {:?}", intermediate_sums_mt);


    /*6*/
    let sum_mt = reduce_data(&intermediate_sums_mt);
    println!("Sum = {}", sum_mt);

}

/*
* CHANGE CODE: code this function
* Note: Don't change the signature of this function
*
* Partitions the data into a number of partitions such that
* - the returned partitions contain all elements that are in the input vector
* - if num_elements is a multiple of num_partitions, then all partitions must have equal number of elements
* - if num_elements is not a multiple of num_partitions, some partitions can have one more element than other partitions
*
* @param num_partitions The number of partitions to create
* @param v The data to be partitioned
* @return A vector that contains vectors of integers
* 
*/
fn partition_data(num_partitions: usize, v: &Vec<usize>) -> Vec<Vec<usize>>{
    // Remove the following line which has been added to remove a compiler error
    //partition_data_in_two(v)

    let mut xn: Vec<Vec<usize>> = Vec::with_capacity(v.len());

    /*The number of vectors that will have an 'odd' size*/
    let fill_remainder = v.len() % num_partitions;

    /*Variable to hold remainder vector size*/
    let remainder_size: usize = ((v.len() as f64) / (num_partitions as f64)).ceil() as usize;

    /*Variable to hold 'normal' size for a vector chunk*/
    let normal_size: usize = ((v.len() as f64) / (num_partitions as f64)).floor() as usize;

    /*Variable to track where we are after the initial remainder iteration*/
    let mut remainder_location = 0;

    for _i in 0..fill_remainder
    {/*First, fill the odd remainder vectors*/

        //println!("Remainder count: {}", remainder_location);
        let mut vec_temp_r : Vec<usize>  = Vec::with_capacity(remainder_size);

        for _j in 0..remainder_size
        {/*Push value from vector onto new vector*/

            vec_temp_r.push(v[remainder_location]);
            remainder_location += 1;
        }

        /*Push vector onto our vector of vectors*/
        xn.push(vec_temp_r.clone());
        assert!(vec_temp_r.capacity() >= remainder_size);
    }

    /*Now fill vector with rest of numbers*/

    for _i in 0..(num_partitions-fill_remainder)
    {/*Continue from where remainder left off, will be 0 if there was no remainder*/
        let mut vec_temp_n : Vec<usize>  = Vec::with_capacity(remainder_size);

        for _j in 0..normal_size
        {

            vec_temp_n.push(v[remainder_location]);
            remainder_location += 1;
        }

        xn.push(vec_temp_n.clone());
        assert!(vec_temp_n.capacity() >= normal_size);
    }

    xn

}