pub fn partition<T>(arr: &mut [T], inicio: usize, fim: usize) -> usize
    where T: std::cmp::PartialOrd
{
    let pivo_indice = fim;
    let mut esquerda: isize = (inicio as isize) - 1;

    for atual in inicio..fim {
        if arr[atual] < arr[pivo_indice] {
            esquerda += 1;
            arr.swap(esquerda as usize, atual);
        }
    }

    arr.swap((esquerda + 1) as usize, fim);

    (esquerda+1) as usize
}

pub fn quicksort_partial<T>(arr: &mut[T], inicio: usize, fim: usize)
    where T: std::cmp::PartialOrd
{
    if inicio < fim {
        let meio = partition(arr, inicio, fim);
        quicksort_partial(arr, inicio, meio-1);
        quicksort_partial(arr, meio+1, fim);
    }
}

pub fn quicksort<T>(arr: &mut[T])
    where T: std::cmp::PartialOrd
{
    quicksort_partial(arr, 0, arr.len()-1);
}


#[cfg(test)]
mod tests {
    use crate::ordenacao::quicksort::{partition, quicksort, quicksort_partial};

    #[test]
    fn test_partition() {
        let mut arr = [1, 9, 5, 7, 12, 3, 2, 10, 11];

        let meio = partition(&mut arr, 2, 7);
        println!("{meio} -> {arr:?}");
        assert_eq!(meio, 6);
        assert_eq!(arr, [1, 9, 5, 7, 3, 2, 10, 12, 11]);
    }

    #[test]
    fn test_quicksort() {
        let mut arr = [1, 9, 5, 7, 12, 3, 2, 10, 11];

        quicksort(&mut arr);
        println!("{arr:?}");
        assert_eq!(arr, [1, 2, 3, 5, 7, 9, 10, 11, 12]);
    }

    #[test]
    fn test_quicksort_partial() {
        let mut arr = [1, 9, 5, 7, 12, 3, 2, 10, 11];

        quicksort_partial(&mut arr, 2, 6);
        println!("{arr:?}");
        assert_eq!(arr, [1, 9, 2, 3, 5, 7, 12, 10, 11]);
    }
}