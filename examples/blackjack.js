const prompt = require("prompt-sync")({ sigint: true });

const CardSuit = {
    CLUBS: 'clubs',
    DIAMONDS: 'diamonds',
    HEARTS: 'hearts',
    SPADES: 'spades'
};

class Card {
    #suit;
    #value;

    constructor(suit, value) {
        this.#suit = suit;
        this.#value = value;
    }

    getSuit() {
        return this.#suit;
    }

    getValue() {
        return this.#value;
    }

    print() {
        console.log(this.getSuit(), this.getValue());
    }
};

class Deck {
    #cards;

    constructor() {
        this.#cards = [];
        for (const suit in CardSuit) {
            for (let value = 1; value <= 13; value++) {
                this.#cards.push(
                    new Card(CardSuit[suit], Math.min(value, 10)));
            }
        }
    }

    print() {
        for (const card of this.#cards) {
            card.print();
        }
    }

    draw() {
        return this.#cards.pop();
    }

    shuffle() {
        for (let i = 0; i < this.#cards.length; i++) {
            const j = Math.floor(Math.random() * 51);
            [this.#cards[i], this.#cards[j]] = [this.#cards[j], this.#cards[i]];
        }
    }
}

class Hand {
    #cards;
    #score;

    constructor() {
        this.#score = 0;
        this.#cards = [];
    }

    addCard(card) {
        this.#cards.push(card);
        if (card.getValue() === 1) {
            this.#score += 11 ? this.#score + 11 <= 21 : 1;
        } else {
            this.#score += card.getValue();
        }
    }

    getScore() {
        return this.#score;
    }

    getCards() {
        return this.#cards;
    }

    print() {
        for (const card of this.getCards()) {
            console.log(card.getSuit(), card.getValue());
        }
    }
}

class Player {
    #hand;

    constructor(hand) {
        this.#hand = hand;
    }

    getHand() {
        return this.#hand;
    }

    clearHand() {
        this.#hand = new Hand();
    }

    addCard(card) {
        this.#hand.addCard(card);
    }

    makeMove() { }
}

class UserPlayer extends Player {
    #balance;

    constructor(balance, hand) {
        super(hand);
        this.#balance = balance;
    }

    getBalance() {
        return this.#balance;
    }

    placeBet(amount) {
        if (amount > this.#balance) {
            throw new Error('Insufficient funds');
        }
        this.#balance -= amount;
        return amount;
    }

    receiveWinnings(amount) {
        this.#balance += amount;
    }

    /** @override */
    makeMove() {
        if (this.getHand().getScore() > 21) {
            return false;
        }
        // read user input
        const move = prompt('Draw card? [y/n] ', 'n');
        return move === 'y';
    }
}

class Dealer extends Player {
    #targetScore;

    constructor(hand) {
        super(hand);
        this.#targetScore = 17;
    }

    updateTargetScore(score) {
        this.#targetScore = score;
    }

    /** @override */
    makeMove() {
        return this.getHand().getScore() < this.#targetScore;
    }
}


class GameRound {
    #player;
    #dealer;
    #deck;

    constructor(player, dealer, deck) {
        this.#player = player;
        this.#dealer = dealer;
        this.#deck = deck;
    }

    #getBetUser() {
        // read integer from user input 
        const amount = prompt('Enter a bet amount ', '0');
        return parseInt(amount);
    }

    #dealInitialCards() {
        for (let i = 0; i < 2; i++) {
            this.#player.addCard(this.#deck.draw());
            this.#dealer.addCard(this.#deck.draw());
        }
        console.log('Player hand: ');
        this.#player.getHand().print()
        const dealerCard = this.#dealer.getHand().getCards()[0];
        console.log('Dealer\'s first card: ');
        dealerCard.print();
    }

    #cleanupRound() {
        this.#player.clearHand();
        this.#dealer.clearHand();
        console.log('Player balance: ', this.#player.getBalance());
    }

    play() {
        this.#deck.shuffle();

        if (this.#player.getBalance() <= 0) {
            console.log('Player has no more money =)');
            return;
        }
        const userBet = this.#getBetUser();
        this.#player.placeBet(userBet);

        this.#dealInitialCards();

        // User makes moves
        while (this.#player.makeMove()) {
            const drawnCard = this.#deck.draw()
            console.log('Player draws', drawnCard.getSuit(), drawnCard.getValue());
            this.#player.addCard(drawnCard);
            console.log('Player score', this.#player.getHand().getScore());
        }
        if (this.#player.getHand().getScore() > 21) {
            console.log('Player loses');
            this.#cleanupRound();
            return;
        }

        // Dealer makes moves
        this.#dealer.updateTargetScore(playerScore);
        while (this.#dealer.makeMove()) {
            this.#dealer.addCard(this.#deck.draw());
        }

        // Determine winner
        const dealerScore = this.#dealer.getHand().getScore();
        if (dealerScore > 21) {
            console.log('Player wins');
            this.#player.receiveWinnings(userBet * 2);
        } else if (dealerScore > playerScore) {
            console.log('Player loses');
        } else {
            console.log('Game ends in a draw');
            this.#player.receiveWinnings(userBet);
        }
        this.#cleanupRound();
    }
}


const player = new UserPlayer(1000, new Hand());
const dealer = new Dealer(new Hand());

while (player.getBalance() > 0) {
    new GameRound(player, dealer, new Deck()).play();
}