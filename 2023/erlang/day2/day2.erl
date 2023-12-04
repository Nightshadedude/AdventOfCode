-module(day2).
-export([part1/1, part2/1,test/0]).

test() ->
    L = ["Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"],
    P = [parse_game(G) || G <- L],
    % erlang:display(P),
    % sum_game_id(P, 0).
    set_power(P, 0).

part1(FileName) ->
    L = parse_line(readlines(FileName)),
    P = [parse_game(G) || G <- L],
    sum_game_id(P, 0).

part2(FileName) ->
    L = parse_line(readlines(FileName)),
    P = [parse_game(G) || G <- L],
    set_power(P, 0).

readlines(FileName) ->
    {ok, Device} = file:open(FileName, [read]),
    try get_all_lines(Device)
      after file:close(Device)
    end.

get_all_lines(Device) ->
    case io:get_line(Device, "") of
        eof  -> [];
        Line -> Line ++ get_all_lines(Device)
    end.

parse_line(String) ->
    string:lexemes(String, "\n").

parse_game(String) ->
    [Game, Dice] = string:split(String, <<": ">>),
    [_, Game_ID] = string:split(Game, " "),
    {Game_ID_Int, _} = string:to_integer(Game_ID),
    % erlang:display({"parse game", Game_ID, Dice}),
    Bag = parse_bag(Dice),
    {Game_ID_Int, Bag}.

parse_bag(Dice) ->
    Rolls = string:split(Dice, <<"; ">>, all),
    % erlang:display({"parse bag", Rolls}),
    RGB = max_roll(Rolls, 0, 0, 0),
    RGB.

max_roll([], R, G, B) ->
    {R, G, B};

max_roll([H|T], R, G, B) ->
    Dice = string:split(H, <<", ">>, all),
    {R1, G1, B1} = parse_roll(Dice, R, G, B),
    R_New = lists:max([R,R1]),
    G_New = lists:max([G,G1]),
    B_New = lists:max([B,B1]),
    max_roll(T, R_New, G_New, B_New).


parse_roll([], R, G, B) ->
    {R, G, B};

parse_roll([H|T], R, G, B) ->
    {Color, Num} = parse_die(H),
    case Color of
        red -> parse_roll(T, Num, G, B);
        green -> parse_roll(T, R, Num, B);
        blue -> parse_roll(T, R, G, Num)
    end.

parse_die(Die) ->
    {Num, Color} = string:to_integer(Die),
    case Color of
        " red" -> {red, Num};
        " green" -> {green, Num};
        " blue" -> {blue, Num}
    end.

sum_game_id([], Acc) ->
    Acc;

sum_game_id([{Game_ID,{R,G,B}}|T], Acc) when R<13, G<14, B<15 ->
    sum_game_id(T, Acc+Game_ID);

sum_game_id([_|T], Acc) ->
    sum_game_id(T, Acc).

set_power([], Acc) ->
    Acc;

set_power([{_,{R,G,B}}|T], Acc) ->
    set_power(T, Acc + (R*G*B)).