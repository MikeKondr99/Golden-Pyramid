---
marp: true
paginate: true
math: katex
theme: iggg
style: img[alt~="center"] { display: block; margin: 0 auto; }
---
<!-- _class: lead -->

# Задача о золотой пирамиде


--- 

# Условие

![bg vertical](https://upload.wikimedia.org/wikipedia/commons/c/ca/1x1.png)

![bg fit](https://d17mnqrx9pmt3e.cloudfront.net/media/blog/share/golden-pyramid-example.svg)

Напишите программу, которая вычисляет наибольшую сумму чисел, расположенных на пути, начинающемся в верхней точке треугольника и заканчивающемся на основании треугольника. 

Каждый шаг на пути может осуществляться вниз по **диагонали влево** или вниз по **диагонали вправо**.

---

# Простое решение

Первое что приходит в голову это перебрать все возможные пути, подсчитать суммы и выбрать максимальную. Например, использовав **рекурсивный** подход.

Но такой алгоритм будет производить много повторяющихся вычислений и иметь сложность $O(2^n)$


![bg vertical](https://upload.wikimedia.org/wikipedia/commons/c/ca/1x1.png)

![bg fit](https://d17mnqrx9pmt3e.cloudfront.net/media/blog/share/golden-pyramid-recursive.svg)

--- 

# Динамический подход

Если использовать методы **динамического программирования**, то можно разбить задачу на подзадачи.

В данном случае нужно разбить на **слои**, для которых произвести вычисления и сохранить промежуточный результат. Алгоритм имеет сложность $O(n)$

<br/>

>>>>>>>>> ![пример h:700](https://cdn.discordapp.com/attachments/1043122497417134115/1043122548080119859/layer.svg)

---

# Пример выполнения

>>>>>> ![](https://d17mnqrx9pmt3e.cloudfront.net/media/blog/share/golden-pyramid-dynamic.svg)

---

# Реализация

На каждой итерации для всех элементов выбираем максимальное значение из пары и складываем.

```rust
pub fn pyramid_simple(mut input: &mut [u32], size: usize) -> u32 {
    if input.len() != (size * (size + 1)) / 2 {
        panic!("Размер данных пирамиды должен соответствовать указному количеству слоёв");
    }
    for i in (2..=size).rev() {
        let (layer, rest) = input.split_at_mut(i);
        for i in 0..layer.len() - 1 {
            rest[i] += layer[i].max(layer[i + 1]);
        }
        input = rest;
    }
    input[0]
}
```

---

# Другой подход

Рассмотрим данный пример с точки зрения функционального программирования
| | | | | | | | 
|-|-|-|-|-|-|-|
|5|6|4|5|6|4|3|
|2|2|2|2|2|2|...|  

|                    |   |    |    |   |   |   |    |   |
|-                   |-  |-   |-   |-  |-  |-  |-   |-  |
| слой$_0^{n-1}$    |   |║ 5 | 6  | 4 | 5 | 6 | 4 ║| 3 |
| слой$_1^n$      | 5 |║ 6 | 4  | 5 | 6 | 4 | 3 ║|   |
| остаток$_0^{n-1}$ |   |║ 2 | 2  | 2 | 2 | 2 | 2 ║|...|

$$ {R'}_0^N = R_0^N +  max(L_0^{n-1},L_1^n)$$

---

# Реализация

Преимущество такого подхода в использовании компилятором векторных операций SIMD(*Single Instruction Multiple Data*)

```rust
pub fn pyramid_vector(mut input: &mut [u32], size: usize) -> u32 {
    if input.len() != (size * (size + 1)) / 2 {
        panic!("Размер данных пирамиды должен соответствовать указному количеству слоёв");
    }
    for i in (2..=size).rev() {
        let (l, r) = input.split_at_mut(i);
        zip!(l[0..n - 1], l[1..], &mut r[0..n - 1]).for_each(|(a, b, r)| *r += a.max(b));
        input = rest;
    }
    input[0]
}

```
---

# Время выполнения

![График](https://cdn.discordapp.com/attachments/1043122497417134115/1043533694872338482/lines.svg)

---

# Вариация 

Расширим задачу для работы не только с пирамидой, но и с **параллелограммом**
```rust
3 2 1 
 4 5 6
  9 8 7
```
* Нужно найти путь сверху вниз, дающий максимальное значение. Начинать можно с любого верхнего числа.

* К числам расположенным с **левого** края всегда прибавляется единственное доступное.

---

# Обобщение

Обобщим динамический метод пирамиды для разных алгоритмов обработки слоя.

```rust
pub fn pyramid<T: LayerCalc>(mut input: &mut [u32], size: usize) -> u32 {
    if input.len() != (size * (size + 1)) / 2 {
        panic!("Размер данных пирамиды должен соответствовать указному количеству слоёв");
    }
    for i in (2..=size).rev() {
        let (layer, rest) = input.split_at_mut(i);
        T::algorithm(layer, &mut rest[0..i - 1], i); // Вызываем алгоритм на слой
        input = rest;
    }
    input[0]
}
```

---

# Параллелограмм

Теперь можно реализовать обобщенный метод для обработки параллелограмма, используя послойные алгоритмы для пирамиды.

```rust
pub fn rectangle<T: LayerCalc>(mut input: &mut [u32], size: usize) -> u32 {
    if input.len() != size * size {
        panic!("Размер данных параллелепипеда должен соответствовать указному количеству слоёв");
    }
    while input.len() > size {
        let (layer, rest) = input.split_at_mut(size);
        T::algorithm(layer, &mut rest[0..size - 1], size);
        rest[size - 1] += layer[size - 1]; // обработка левого элемента
        input = rest;
    }
    *input.iter().max().expect("ввод не должен быть пустым")
}
```

---

# Заключение

Была изучена задача **"Золотая пирамида"**. 
Рассмотрены варианты её решения:
* Простой
* Динамический
* Динамический с SIMD

Изучена вариация данной задачи для параллелограмма.






