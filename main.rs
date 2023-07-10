extern crate gtk;
use gtk::prelude::*;
use gtk::{Button, Window, WindowType};

fn main() {
    // Инициализация GTK+
    gtk::init().expect("Failed to initialize GTK+");

    // Создание окна
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Привет, мир!");
    window.set_default_size(350, 70);

    // Создание кнопки
    let button = Button::new_with_label("Нажми меня!");

    // Добавление кнопки в окно
    window.add(&button);

    // Обработчик события нажатия на кнопку
    button.connect_clicked(|_| {
        println!("Кнопка нажата!");
    });

    // Обработчик события закрытия окна
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Отображение окна и запуск главного цикла обработки событий
    window.show_all();
    gtk::main();
}
