-module(day4).
-export([part1/1, part2/1,test/0]).

test() ->
    test_shape_does_not_match_input_shape.

part1(FileName) ->
    L = parse_line(readlines(FileName)),
    Cards = [parse_card(PC) || PC <- L],
    lists:sum([win_calc(C) || C <- Cards]).

part2(FileName) ->
    L = parse_line(readlines(FileName)),
    Cards = [parse_card(PC) || PC <- L],
    maybe_stack(Cards, Cards, 0).

readlines(FileName) ->
    {ok, Data} = file:read_file(FileName),
    % erlang:display(Data),
    Data.

parse_line(Data) ->
    binary:split(Data, <<10>>, [global]).

parse_card(Data) ->
<<_:40/bitstring, Game_ID:24/bitstring, _:8/bitstring, LNum1:24/bitstring, LNum2:24/bitstring, LNum3:24/bitstring, LNum4:24/bitstring, LNum5:24/bitstring, LNum6:24/bitstring, LNum7:24/bitstring, LNum8:24/bitstring, LNum9:24/bitstring, LNum10:24/bitstring, _:16/bitstring, RNum1:24/bitstring, RNum2:24/bitstring, RNum3:24/bitstring, RNum4:24/bitstring, RNum5:24/bitstring, RNum6:24/bitstring, RNum7:24/bitstring, RNum8:24/bitstring, RNum9:24/bitstring, RNum10:24/bitstring, RNum11:24/bitstring, RNum12:24/bitstring, RNum13:24/bitstring, RNum14:24/bitstring, RNum15:24/bitstring, RNum16:24/bitstring, RNum17:24/bitstring, RNum18:24/bitstring, RNum19:24/bitstring, RNum20:24/bitstring, RNum21:24/bitstring, RNum22:24/bitstring, RNum23:24/bitstring, RNum24:24/bitstring, RNum25:24/bitstring>> = Data,
% <<_:40/bitstring, Game_ID:24/bitstring, _:8/bitstring, LNum1:24/bitstring, LNum2:24/bitstring, LNum3:24/bitstring, LNum4:24/bitstring, LNum5:24/bitstring, LNum6:24/bitstring, LNum7:24/bitstring, LNum8:24/bitstring, LNum9:24/bitstring, LNum10:24/bitstring, _:16/bitstring, Rest/bitstring>> = Data,
% erlang:display({gameid, Game_ID, lhs, [LNum1, LNum2], rest, Rest}),
 
{list_to_integer(string:strip(binary_to_list(Game_ID),left)),
[list_to_integer(string:strip(binary_to_list(LNum1),left)),
list_to_integer(string:strip(binary_to_list(LNum2),left)),
list_to_integer(string:strip(binary_to_list(LNum3),left)),
list_to_integer(string:strip(binary_to_list(LNum4),left)),
list_to_integer(string:strip(binary_to_list(LNum5),left)),
list_to_integer(string:strip(binary_to_list(LNum6),left)),
list_to_integer(string:strip(binary_to_list(LNum7),left)),
list_to_integer(string:strip(binary_to_list(LNum8),left)),
list_to_integer(string:strip(binary_to_list(LNum9),left)),
list_to_integer(string:strip(binary_to_list(LNum10),left))
],
[list_to_integer(string:strip(binary_to_list(RNum1),left)),
list_to_integer(string:strip(binary_to_list(RNum2),left)),
list_to_integer(string:strip(binary_to_list(RNum3),left)),
list_to_integer(string:strip(binary_to_list(RNum4),left)),
list_to_integer(string:strip(binary_to_list(RNum5),left)),
list_to_integer(string:strip(binary_to_list(RNum6),left)),
list_to_integer(string:strip(binary_to_list(RNum7),left)),
list_to_integer(string:strip(binary_to_list(RNum8),left)),
list_to_integer(string:strip(binary_to_list(RNum9),left)),
list_to_integer(string:strip(binary_to_list(RNum10),left)),
list_to_integer(string:strip(binary_to_list(RNum11),left)),
list_to_integer(string:strip(binary_to_list(RNum12),left)),
list_to_integer(string:strip(binary_to_list(RNum13),left)),
list_to_integer(string:strip(binary_to_list(RNum14),left)),
list_to_integer(string:strip(binary_to_list(RNum15),left)),
list_to_integer(string:strip(binary_to_list(RNum16),left)),
list_to_integer(string:strip(binary_to_list(RNum17),left)),
list_to_integer(string:strip(binary_to_list(RNum18),left)),
list_to_integer(string:strip(binary_to_list(RNum19),left)),
list_to_integer(string:strip(binary_to_list(RNum20),left)),
list_to_integer(string:strip(binary_to_list(RNum21),left)),
list_to_integer(string:strip(binary_to_list(RNum22),left)),
list_to_integer(string:strip(binary_to_list(RNum23),left)),
list_to_integer(string:strip(binary_to_list(RNum24),left)),
list_to_integer(string:strip(binary_to_list(RNum25),left))]
}.


win_calc(Card) ->
    {_, L, R} = Card,
    {W, _ } = lists:partition(fun(X) -> lists:member(X,R) end, L),
    trunc(math:pow(2,length(W)-1)).

win_calc_p2(Card) ->
    {_, L, R} = Card,
    {W, _ } = lists:partition(fun(X) -> lists:member(X,R) end, L),
    length(W).

maybe_stack([], _, Num_Acc) ->
    Num_Acc;

maybe_stack([H|T], Cards, Num_Acc) ->
    {Game_ID, _, _} = H,
    erlang:display({current_game, Game_ID, queue_depth, length(T)}),
    Wins = win_calc_p2(H),
    Add_Cards = lists:sublist(Cards, Game_ID + 1, Wins),
    New_Tail = lists:flatten([Add_Cards|[T]]),
    maybe_stack(New_Tail, Cards, Num_Acc+1).