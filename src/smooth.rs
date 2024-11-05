use glissade::Mix;

pub fn smooth<T: Mix + Clone + Copy>(items: Vec<T>) -> Vec<T> {
    (0..items.len())
        .map(move |i| {
            if i == 0 || i == items.len() - 1 {
                items[i]
            } else {
                items[i - 1].mix(items[i + 1], 0.5)
            }
        })
        .collect()
}
