from enum import Enum
import os
import sys
import random


# a card is a tuple of (Suit,Value)
class Suit(Enum):
    HEARTS = 14
    SPADES = 15
    CLUBS = 16
    DIAMONDS = 17

# start off with small number of cards (16) for testing purposes
class Value(Enum):
    ACE = 1
    TWO = 2
    THREE = 3
    FOUR = 4

# ############################################################################

# pseudo code for go fish

# object for a game
class Game:
    def __init__(self):
        # ordered list of players
        self.playerlist = []

    def add_player(self, player):
        #index into the player list
        self.playerlist.append(player)
        print(f'Player {len(self.playerlist)+1} has been added')
           
class Player:
    def __init__(self, name):
        self.name = name
        # define the id of the player
        self.id = 0
        # hand is a dictionary key is rank, list of suits
        self.hand = {}
        self.points = 0

    # functions to manipulate player values
    def receive_card(self,card):
        # suit is a string
        suit = card[0]
        print(suit)
        # value is the numerical value for the card
        value = card[1]
        if value in self.hand:
            self.hand[value].append(suit)
        else:
            self.hand[value] = [suit]

# deals n cards from the deck to add to the player's hand
def deal_cards(deck, player, n):
    if n < len(deck):
        #make sure that there are no sizing conflicts
        for i in range(n):
            card = deck.pop()
            player.receive_card(card)

def print_deck(deck):
    for i in range(len(deck)):
        card = deck[i]
        print(f'{card[1]} of {card[0]}')
    print('\n')

# pass in the player's hand (a dictionary)
def print_hand(hand):
    for rank in hand:
        for suit in hand[rank]:
            print(f'{rank} of {suit}')
    print('\n')

# create the deck (shuffled)
# a card is a tuple of (Suit,Value)
# a deck is an array of tuples
def make_shuffled_deck():
    deck = []
    for s in Suit:
        for v in Value:
            card = (s.name, v.value)
            deck.append(card)
            # print the card
            print(f'{v.value} of {s.name}')
            # print(f'{type(v.value)} of {type(s.name)}')

    random.shuffle(deck)
    return deck

def main():
    # decide who goes first, change this to something like a coin flip later on

    # ################# Initialize Game #################################
    # Add options to specify how many players per lobby
    print("You can decide who goes first by who is Player 1 and who is Player 2.")
    p1_name = input("Player 1 name: ")
    p2_name = input("Player 2 name: ")

    p1 = Player(p1_name)
    p2 = Player(p2_name)

    game = Game()
    game.add_player(p1)
    game.add_player(p2)

    start_game = False
    while(not start_game):
        game_ready = input("Ready to start the game? Enter 'y' to begin: ")
        if (game_ready == 'y'):
            start_game = True
            break
        else:
            continue

    # MAKE the deck 
    deck = make_shuffled_deck()
    print(f'Deck size: {len(deck)} \n')
    print("Shuffled Deck:")
    print_deck(deck)

    # DEAL starting hand
    deal_cards(deck, p1, 4)
    deal_cards(deck, p2, 4)
    print("Player1's hand:")
    print_hand(p1.hand)
    print("Player2's hand:")
    print_hand(p2.hand)
    print("Current deck:")
    print_deck(deck)
    
    game_over = False
    player_turn = 0

    
    # ##################################################
    while(not game_over):

        turn_over = False

        while(not turn_over):

            curr_player = game.playerlist[player_turn]

            if player_turn == len(game.playerlist) - 1:
                opp_player = 0
            else:
                opp_player = game.playerlist[player_turn+1]

            # print all the cards in player's hand
            print(f'{curr_player.name} (Player {player_turn+1}) hand: ')
            print_hand(curr_player.hand)

            # ask player to make matches
            # value_match = input("Any value matches? Type value to make a set")
            
            # check_match(curr_player, value_match)

            # Assume only 2 players here
            requested_value = input("It's your move! \n What value are you asking for? (1-13)")
            # check if opponent has the card
            # if (requested_value in opp_player.hand) and len(opp_player.hand[requested_value]) > 0:
                # check if any card in p2's hand has this value


            answ= input("Done with your turn? 'y' for yes, 'n' for no")
            if (answ == 'y'):
                turn_over = True
            else:
                continue

        # end of turn go back to player 1
        player_turn += 1
        if player_turn == len(game.playerlist):
            player_turn = 0
    
    # ##################################################



if __name__ == '__main__':
    main()
