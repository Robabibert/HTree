use num::Float;
use std::marker::PhantomData;
use std::convert::From;
const SCALE_HEIGHT: f64 = 0.7071067811865475244;

#[derive(Clone, Copy, Debug)]
pub struct HTree<T> {
    order: usize,
    _marker: PhantomData<T>,
}

pub struct HTreeIterator<T>
where
    T: Float,
{
    h_tree: HTree<T>,
    index: usize,
}

impl<T> HTree<T>
where
    T: Float,
{
    pub fn new(order: usize) -> HTree<T> {
        HTree {
            order,
            _marker: PhantomData {},
        }
    }
}
impl<T> Iterator for HTreeIterator<T>
where
    T: Float,
{
    type Item = ((T, T), (T, T));
    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        let order_index = self.index.ilog2() as u32;
        if order_index > self.h_tree.order as u32 {
            return None;
        }
        let iteration_index = self.index as u32 - (1u32 << order_index);

        let num_vertical_rectangles = 1u32 << (order_index + 1) / 2;
        let num_horizontal_rectangles = 1u32 << order_index / 2 + 1;
        let num_rectangles = num_vertical_rectangles * num_horizontal_rectangles;
        assert_eq!(num_rectangles >= iteration_index * 2, true);

        let rectangle_index = 2 * iteration_index;
        let num_x_start;
        let num_y_start;
        let num_x_end;
        let num_y_end;
        if order_index % 2 == 1 {
            // direction ==1 -> vertical
            //iteration_index=y+height*x
            num_y_start = rectangle_index % num_vertical_rectangles;
            num_x_start = (rectangle_index - num_y_start) / num_vertical_rectangles;
            num_y_end = (rectangle_index + 1) % num_vertical_rectangles;
            num_x_end = ((rectangle_index + 1) - num_y_end) / num_vertical_rectangles;
        } else {
            // direction ==0 -> horizontal
            //iteration_index=x+width*y
            num_x_start = rectangle_index % num_horizontal_rectangles;
            num_y_start = (rectangle_index - num_x_start) / num_horizontal_rectangles;
            num_x_end = (rectangle_index + 1) % num_horizontal_rectangles;
            num_y_end = ((rectangle_index + 1) - num_x_end) / num_horizontal_rectangles;
        }

        let x_start: T = (T::from(num_x_start).unwrap() + T::from(0.5).unwrap())
            / T::from(num_horizontal_rectangles).unwrap();
        let x_end: T = (T::from(num_x_end).unwrap() + T::from(0.5).unwrap())
            / T::from(num_horizontal_rectangles).unwrap();
        let y_start: T = (T::from(num_y_start).unwrap() + T::from(0.5).unwrap())
            / T::from(num_vertical_rectangles).unwrap();
        let y_end: T = (T::from(num_y_end).unwrap() + T::from(0.5).unwrap())
            / T::from(num_vertical_rectangles).unwrap();
        Some((
            (x_start, y_start * T::from(SCALE_HEIGHT).unwrap()),
            (x_end, y_end * T::from(SCALE_HEIGHT).unwrap()),
        ))
    }
}

impl<T> IntoIterator for HTree<T>
where
    T: Float,
{
    type Item = ((T, T), (T, T));
    type IntoIter = HTreeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        HTreeIterator {
            h_tree: self,
            index: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HTree;

    #[test]
    fn collect_htree_f32() {
        let order = 2;
        let htree: HTree<f32> = HTree::new(2);
        let lines: Vec<((f32, f32), (f32, f32))> = htree.into_iter().collect();
        let j = 0;
    }
    #[test]
    fn collect_htree_f64() {
        let order = 2;
        let htree: HTree<f64> = HTree::new(2);
        let lines: Vec<((f64, f64), (f64, f64))> = htree.into_iter().collect();
        let j = 0;
    }
}
