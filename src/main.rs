mod computer;
mod solution_template;
mod solutions;

use futures::executor::block_on;
use futures::future::join_all;
use solution_template::Solution;
use solutions::answers::AnswerSolution;
use solutions::bags::BagSolution;
use solutions::calculator::CalculatorSolution;
use solutions::conway::ConwaySolution;
use solutions::ferry::FerrySolution;
use solutions::game::GameSolution;
use solutions::halting::HaltingSolution;
use solutions::jigsaw::JigsawSolution;
use solutions::jolts::JoltSolution;
use solutions::masks::MaskSolution;
use solutions::messages::MessageSolution;
use solutions::passports::PassportSolution;
use solutions::passwords::PasswordSolution;
use solutions::rain::RainSolution;
use solutions::seats::SeatSolution;
use solutions::shuttles::ShuttleSolution;
use solutions::sum_2020::Sum2020Solution;
use solutions::tickets::TicketSolution;
use solutions::trees::TreeSolution;
use solutions::xmas::XmasSolution;

fn main() {
    block_on(join_all(vec![
        Sum2020Solution::run("2020_sums.txt"),
        PasswordSolution::run("passwords.txt"),
        TreeSolution::run("trees.txt"),
        PassportSolution::run("passports.txt"),
        SeatSolution::run("seats.txt"),
        AnswerSolution::run("answers.txt"),
        BagSolution::run("bags.txt"),
        HaltingSolution::run("halting.txt"),
        XmasSolution::run("xmas.txt"),
        JoltSolution::run("jolts.txt"),
        FerrySolution::run("ferry.txt"),
        RainSolution::run("rain.txt"),
        ShuttleSolution::run("shuttles.txt"),
        MaskSolution::run("masks.txt"),
        GameSolution::run("game.txt"),
        TicketSolution::run("tickets.txt"),
        ConwaySolution::run("conway.txt"),
        CalculatorSolution::run("calculator.txt"),
        MessageSolution::run("messages.txt"),
        JigsawSolution::run("jigsaw.txt"),
    ]));
}
