use std::env;
use std::thread;

fn print_partition_info(vs: &Vec<Vec<usize>>){
    println!("Number of partitions = {}\n", vs.len());
    for i in 0..vs.len(){
        println!("\tsize of partition {} = {}", i, vs[i].len());
    }
}

fn generate_data(num_elements: usize) -> Vec<usize>{
    let mut v : Vec<usize> = Vec::new();
    for i in 0..num_elements {
        v.push(i);
    }
    return v;
}

fn partition_data_in_two(v: &Vec<usize>) -> Vec<Vec<usize>>{
    let partition_size = v.len() / 2;
    let mut xs: Vec<Vec<usize>> = Vec::new();

    let mut x1 : Vec<usize> = Vec::new();
    for i in 0..partition_size{
        x1.push(v[i]);
    }
    xs.push(x1);

    let mut x2 : Vec<usize> = Vec::new();
    for i in partition_size..v.len(){
        x2.push(v[i]);
    }
    xs.push(x2);
    xs
}

fn map_data(v: &Vec<usize>) -> usize{
    let mut sum = 0;
    for i in v{
        sum += i;
    }
    sum
}

fn reduce_data(v: &Vec<usize>) -> usize{
    let mut sum = 0;
    for i in v{
        sum += i;
    }
    sum
}


fn main() {

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

    let v = generate_data(num_elements);

    let xs = partition_data_in_two(&v);

    print_partition_info(&xs);

    let mut intermediate_sums : Vec<usize> = Vec::new();

    let part1 = xs[0].clone();
    let t1 = thread::spawn(move || {map_data(&part1)});
    let part2 = xs[1].clone();
    let t2 = thread::spawn(move || {map_data(&part2)});
    
    let res1 = t1.join().unwrap();
    let res2 = t2.join().unwrap();
    
    intermediate_sums.push(res1);
    intermediate_sums.push(res2);

    println!("Intermediate sums = {:?}", intermediate_sums);

    let sum = reduce_data(&intermediate_sums);
    println!("Sum = {}\n", sum);

    intermediate_sums.clear();
    //println!("PIE");
    let _ks = partition_data(num_partitions,&v); 
    let nn = 0;
    let mut vv=Vec::new();
    for _i in 0..num_partitions{
      let p2 = _ks[nn].clone();
      let tt2 = thread::spawn(move || {map_data(&p2)});
      
      vv.push(tt2)
    }
    for entry in vv{
        intermediate_sums.push(entry.join().unwrap());
    }

    println!("Intermediate sums = {:?}", intermediate_sums);
    let fin = reduce_data(&intermediate_sums);
    println!("Final Sum =  {:?}\n", fin);

}

fn increment(n: &mut usize) {
    *n += 1;
}

fn partition_data(num_partitions: usize, v: &Vec<usize>) -> Vec<Vec<usize>>{
    println!("Number of Partitions: {:?}\n", num_partitions);
    //println!("VECTOR {:?}", v.len());
    let partition_size = v.len() / num_partitions;
    let ex = v.len() % num_partitions;
    let mut ss = 0;
    let mut j = 0;
    for i in 0..num_partitions{
        if i < ex{
          println!("\tsize of partition {} = {}", i, partition_size+1);
        }
        else{
          println!("\tsize of partition {} = {}", i, partition_size);
        }
    }

    let mut ks: Vec<Vec<usize>> = Vec::new();
    let mut x_y : Vec<usize> = Vec::new();

    
    for _k in 0..num_partitions-1{
      let mut x_i : Vec<usize> = Vec::new();
      for _i in 0..partition_size{
        x_y.push(v[j]);
        if ss!=partition_size{
          x_i.push(v[ss]);
          increment(&mut ss);
          if j!=ex{
            x_i.push(v[ss]);
            increment(&mut ss);
            j+=1;
            x_y.push(v[j]);
          }
          else if j==ex{
            x_i.push(v[ss]);
            if ss==partition_size{
              x_y.push(v[j]);
              increment(&mut ss);
            }
            x_y.push(v[j]);
          }
        }       
        else if ss==partition_size{
          println!("SD {}",ss);
          x_i.push(v[ss]);
          x_y.push(v[j]);
          j+=1;
        }
      }
      
      ks.push(x_i);
    }
    //ks.push(x_y);
    /* This has correct formatting but dosnt work for big numbers
    works for ./main 3 4 correctly
      for _k in 0..num_partitions{
      let mut x_i : Vec<usize> = Vec::new();
      for _i in 0..partition_size{
        x_y.push(v[j]);
        if ss!=partition_size{
          x_i.push(v[ss]);
          increment(&mut ss);
          if j!=ex{
            x_i.push(v[ss]);
            increment(&mut ss);
            j+=1;
            x_y.push(v[j]);
          }
          else if j==ex{
            x_y.push(v[j]);
          }
        }       
        else if ss==partition_size{
          //println!("SD {}",ss);
          x_i.push(v[ss]);
          x_y.push(v[j]);
          j+=1;
        }
      }
      
      ks.push(x_i);
    }
    */
  

    ks
}

