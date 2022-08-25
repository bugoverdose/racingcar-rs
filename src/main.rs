use crate::car::Car;
use crate::game::Game;

mod car;
mod game;
mod view;

fn main() {
    println!("경주할 자동차 이름을 입력하세요.(이름은 쉼표(,) 기준으로 구분)");
    let car_names: Vec<String> = view::read_car_names_input();
    let total_rounds = view::read_total_rounds_input();

    let mut game = Game::new(car_names);
    println!("실행결과");
    loop {
        if game.is_over(total_rounds) {
           break
        }
        game = game.play();
        for car in &game.cars {
            println!("{} : {}", car.name, "-".repeat(car.distance as usize));
        }
        println!();
    }

    println!("최종 우승자 : {}", game.get_winners().join(", "))
}
