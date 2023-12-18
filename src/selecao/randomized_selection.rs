use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use rand::Rng;

fn randomized_partition<T>(arr: &mut [T], inicio: usize, fim: usize) -> usize
    where T: std::cmp::PartialOrd + Display + Debug
{
    let pivo_indice = rand::thread_rng().gen_range(inicio..fim+1);
    println!("pivo: {} -> {:?}", arr[pivo_indice], arr);
    arr.swap(pivo_indice, fim);
    crate::ordenacao::quicksort::partition(arr, inicio, fim)
}

fn randomized_select_partial<T>(arr: &mut [T], _inicio: usize, _fim: usize, position: usize) -> usize
    where T: std::cmp::PartialOrd + Display + Debug
{
    let meio = randomized_partition(arr, _inicio, _fim);
    match position.cmp(&meio) {
        Ordering::Less    => { randomized_select_partial(arr, _inicio, meio-1, position) }
        Ordering::Equal   => { meio }
        Ordering::Greater => { randomized_select_partial(arr, meio+1, _fim, position) }
    }
}

fn randomized_select<T>(arr: &mut [T], position: usize) -> &T
    where T: std::cmp::PartialOrd + Display + Debug
{
    randomized_select_partial(arr, 0, arr.len()-1, position);
    &arr[position]
}

#[cfg(test)]
mod tests {
    use crate::selecao::randomized_selection::randomized_select;

    #[test]
    fn test_randomized_select() {
        let mut arr = [1, 9, 5, 7, 12, 3, 2, 10, 11];

        let valor = randomized_select(&mut arr, 6).clone();
        println!("{valor} -> {arr:?}");
        assert_eq!(valor, 10);
    }
}