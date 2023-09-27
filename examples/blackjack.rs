use rand::Rng;
use std::{cell::RefCell, cmp::min, io::stdin, rc::Rc};
// BlackJack 棋牌游戏游戏: 二十一点
// 游戏介绍:
// 21点又名黑杰克（Blackjack），起源于法国，已流传到世界各地，有着悠久的历史。
// 在世界各地的赌场中都可以看到二十一点，随着互联网的发展，二十一点开始走向网络时代。
// 该游戏由2到6个人玩，使用除 大小王 之外的52张牌，游戏者的目标是使手中的牌的点数之和不超过21点且尽量大。
//
// 大家手中扑克点数的计算是：2至9牌，按其原点数计算；K、Q、J和10牌都算作10点（一般记作T，即ten之意）；
// A 牌（ace）既可算作1点也可算作11点，由玩家自己决定（当玩家停牌时，点数一律视为最大而尽量不爆，
// 如A+9为20，A+4+8为13，A+3+A视为15）
//
// 开局时，庄家（dealer）给每个玩家（又称闲家）牌面向上发两张牌（明牌），再给庄家自己发两张牌，一张明牌，一张暗牌（牌面朝下）
// 当所有的初始牌分发完毕后，如果玩家拿到的是A和T（无论顺序），就拥有黑杰克（Black Jack）；
//
// 若庄家的明牌为T，且暗牌为A，应直接翻开并拥有Black Jack；
// 如果庄家的明牌为A，则玩家可以考虑买不买保险（Insurance），保险金额是赌注的一半且不退。
// 此时，如果庄家的暗牌为10点牌（构成Black Jack），那么翻开此牌，购买保险的玩家得到1倍赌注；
// 如果庄家没有Black Jack则保持暗牌，玩家继续游戏。若玩家为Black Jack且庄家为其他，
// 玩家赢得1.5倍（或2倍，1赔2时）赌注；若庄家为Black Jack且玩家为其他，庄家赢得赌注；若庄家和玩家均为Black Jack，平局，玩家拿回自己的赌注。
// 接下来是正常的拿牌流程：首名非黑杰克玩家选择拿牌（Hit）、停牌（Stand）、加倍（Double）、分牌（Split，两牌相同时）或投降（Surrender，庄家赢得一半赌注）；若选择拿牌，则后续只能选择拿牌或停牌。
// 在发牌的过程中，如果玩家的牌点数的和超过21，玩家就输了——叫爆掉（Bust），庄家赢得赌注（无论庄家之后的点数是多少）。
// 假如玩家没爆掉，又决定不再要牌了（停牌，或因加倍、投降而终止），则轮到下一名非黑杰克玩家选择。
// 当所有玩家停止拿牌后，庄家翻开暗牌，并持续拿牌直至点数不小于17（若有A，按最大而尽量不爆计算）。
// 假如庄家爆掉了，那他就输了，玩家赢得1倍赌注；否则那么比点数大小，大为赢。点数相同为平局，玩家拿回自己的赌注。

/// Suit 扑克牌的花色
/// 扑克牌中的四种花色，即黑桃(spade)、红桃(heart)、梅花(club)、方块(dianmond)，代表一年中的春夏秋冬四季，
/// 而每种花色刚好13张，指每个季节有13个星期
#[derive(Debug, Clone, Copy)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    // 用于遍历枚举
    const SUIT: [Self; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suit = match self {
            Suit::Clubs => "clubs",
            Suit::Diamonds => "diamonds",
            Suit::Hearts => "hearts",
            Suit::Spades => "spades",
        };
        write!(f, "{}", suit)
    }
}

/// 扑克牌
#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub value: u8,
}

impl Card {
    pub fn new(suit: Suit, value: u8) -> Self {
        Self { suit, value }
    }

    pub fn print(&self) {
        println!("{}", self)
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.suit, self.value)
    }
}

/// 一副扑克牌
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for suit in Suit::SUIT {
            for value in 1..14 {
                cards.push(Card::new(suit, min(value, 10)));
            }
        }
        Self { cards }
    }

    pub fn print(&self) {
        for card in &self.cards {
            card.print();
        }
    }

    pub fn draw(&mut self) -> Card {
        self.cards.pop().unwrap()
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.cards.len() {
            let j = rng.gen_range(0..=51);
            self.cards.swap(i, j);
        }
    }
}

/// 每个人手里持有的扑克牌
#[derive(Debug, Clone)]
pub struct Hand {
    pub score: u8,
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Hand {
            score: 0,
            cards: Vec::new(),
        }
    }

    pub fn add_card(&mut self, card: Card) {
        let card_clone = card.clone();
        self.cards.push(card_clone);
        if card.value == 1 {
            // 碰到 'A', A可以被认为是1点或11点
            if self.score + 11 <= 21 {
                // 如果相加小于等于21点(没越界), 那么 A 就当做11点
                self.score += 11;
            } else {
                // 如果相加大于21点(越界), 那么 A 就当做1点
                self.score += 1;
            }
        } else {
            self.score += card.value
        }
        println!("Score: {}", self.score)
    }

    pub fn print(&self) {
        for card in &self.cards {
            card.print();
        }
    }
}

pub trait PlayerMakeMove {
    fn make_move(&self) -> bool;
}

pub struct Player {
    pub hand: Hand,
}

impl Player {
    pub fn new(hand: Hand) -> Self {
        Self { hand }
    }

    pub fn clear_hand(&mut self) {
        self.hand = Hand::new();
    }

    pub fn add_card(&mut self, card: Card) {
        self.hand.add_card(card);
    }

    pub fn get_score(&self) -> u8 {
        self.hand.score
    }
}

/// 玩家
pub struct UserPlayer {
    player: Player,
    balance: u64,
}

impl UserPlayer {
    pub fn new(hand: Hand, balance: u64) -> Self {
        Self {
            player: Player::new(hand),
            balance,
        }
    }
    pub fn add_card(&mut self, card: Card) {
        self.player.add_card(card);
    }

    pub fn clear_hand(&mut self) {
        self.player.clear_hand();
    }

    pub fn get_balance(&self) -> u64 {
        self.balance
    }

    pub fn get_hand(&self) -> Hand {
        self.player.hand.clone()
    }

    pub fn get_score(&self) -> u8 {
        self.player.get_score()
    }

    pub fn place_bet(&mut self, amount: u64) {
        if amount > self.balance {
            todo!("panic")
        }
        self.balance -= amount;
    }

    pub fn receive_winnings(&mut self, amount: u64) {
        self.balance += amount;
    }
}

impl PlayerMakeMove for UserPlayer {
    fn make_move(&self) -> bool {
        if self.player.hand.score > 21 {
            return false;
        }

        println!("Draw card? [y/n] ");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        input == "y"
    }
}

/// 庄家
pub struct Dealer {
    player: Player,
    target_score: u8,
}

impl Dealer {
    pub fn new(hand: Hand) -> Self {
        Self {
            player: Player::new(hand),
            target_score: 17,
        }
    }
    pub fn add_card(&mut self, card: Card) {
        self.player.add_card(card);
    }

    pub fn get_hand(&self) -> Hand {
        self.player.hand.clone()
    }

    pub fn get_score(&self) -> u8 {
        self.player.get_score()
    }

    pub fn clear_hand(&mut self) {
        self.player.clear_hand();
    }

    pub fn update_target_scroe(&mut self, score: u8) {
        self.target_score = score;
    }
}

impl PlayerMakeMove for Dealer {
    fn make_move(&self) -> bool {
        self.player.hand.score < self.target_score
    }
}

pub struct GameRound {
    player: Rc<RefCell<UserPlayer>>, // 玩家
    dealer: Rc<RefCell<Dealer>>,     // 庄家
    deck: Deck,                      // 牌桌
}

impl GameRound {
    pub fn new(player: Rc<RefCell<UserPlayer>>, dealer: Rc<RefCell<Dealer>>, deck: Deck) -> Self {
        Self {
            player,
            dealer,
            deck,
        }
    }

    pub fn player_has_money(&self) -> bool {
        self.player.borrow().get_balance() > 0
    }

    pub fn play(&mut self) {
        self.deck.shuffle();
        if self.player.borrow().get_balance() <= 0 {
            println!("Player has no more money =)");
            return;
        }

        let user_bet = self.get_bet_user();
        self.player.borrow_mut().place_bet(user_bet);
        self.deal_initial_cards();

        // User makes moves
        while self.player.borrow_mut().make_move() {
            let drawn_card = self.deck.draw();
            let drawn_card_clone = drawn_card.clone();
            println!("Player draws ({} {})", drawn_card.suit, drawn_card.value);
            self.player.borrow_mut().add_card(drawn_card_clone);
            println!("Player score {}", self.player.borrow().get_hand().score);
        }
        if self.player.borrow().get_score() > 21 {
            println!("Player loses");
            self.cleanup_round();
            return;
        }
        let player_score = self.player.borrow().get_score();

        // Dealer makes moves
        self.dealer.borrow_mut().update_target_scroe(player_score);
        while self.dealer.borrow().make_move() {
            self.dealer.borrow_mut().add_card(self.deck.draw());
        }

        // Determine winner
        let dealer_score = self.dealer.borrow().get_score();
        if dealer_score > 21 {
            println!("Player wins");
            self.player.borrow_mut().receive_winnings(user_bet * 2);
        } else if dealer_score > player_score {
            println!("Player loss");
        } else {
            println!("Game ends in a draw");
            self.player.borrow_mut().receive_winnings(user_bet);
        }
        self.cleanup_round();
    }

    fn cleanup_round(&mut self) {
        self.player.borrow_mut().clear_hand();
        self.dealer.borrow_mut().clear_hand();
        println!("Player balance: {}", self.player.borrow().balance);
    }

    fn get_bet_user(&self) -> u64 {
        println!("Enter a bet amount: ");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim().parse::<u64>().unwrap_or(0u64);
        input
    }

    fn deal_initial_cards(&mut self) {
        for _ in 0..2 {
            self.player.borrow_mut().add_card(self.deck.draw());
            self.dealer.borrow_mut().add_card(self.deck.draw());
        }
        println!("Player hand: ");
        self.player.borrow().get_hand().print();
        let dealer_card = self.dealer.borrow().get_hand().cards[0];
        println!("Dealer's first card: ");
        dealer_card.print();
    }
}

fn main() {
    let player = Rc::new(RefCell::new(UserPlayer::new(Hand::new(), 1000)));
    let dealer = Rc::new(RefCell::new(Dealer::new(Hand::new())));
    while player.borrow().balance > 0 {
        let player = Rc::clone(&player);
        let dealer = Rc::clone(&dealer);
        let mut game = GameRound::new(player, dealer, Deck::new());
        game.play();
    }
}
