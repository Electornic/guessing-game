use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    loop {
        // 게임 시작 메시지 및 범위 안내
        println!("숫자 맞추기 게임!");
        println!("1부터 100 사이의 숫자를 맞춰보세요!");

        // 1부터 100까지 포함하도록 수정 (1..=100)
        let secret_number = rand::thread_rng().gen_range(1..=100);
        let mut attempts = 0;
        const MAX_ATTEMPTS: u32 = 10;

        // 게임 루프
        loop {
            attempts += 1;
            let remaining = MAX_ATTEMPTS - attempts + 1;

            // 남은 기회 표시
            println!("남은 기회: {}번", remaining);
            println!("숫자를 입력해주세요:");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            // 더 친절한 입력값 검증
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("숫자를 입력해주세요!");
                    attempts -= 1; // 잘못된 입력은 시도 횟수에 포함하지 않음
                    continue;
                },
            };

            println!("당신이 추측한 숫자: {}", guess);

            // 추측 결과 확인 (컬러풀한 출력)
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("{}", "Too small!".blue()),
                Ordering::Greater => println!("{}", "Too big!".red()),
                Ordering::Equal => {
                    println!("{}", "You win!".green());
                    break;
                },
            }

            // 시도 횟수 제한 확인
            if attempts >= MAX_ATTEMPTS {
                println!("{}", format!("실패! 정답은 {}였습니다.", secret_number).bright_black());
                break;
            }
        }

        // 다시 플레이 기능
        println!("다시 플레이하시겠습니까? (y/n)");
        let mut play_again = String::new();
        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");

        match play_again.trim().to_lowercase().as_str() {
            "y" => continue, // 새 게임 시작
            _ => {
                println!("{}", "게임을 종료합니다.".bright_black());
                break; // 프로그램 종료
            }
        }
    }
}
