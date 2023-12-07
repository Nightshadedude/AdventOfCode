-module(day3).
-export([part1/1, part2/1,test/0]).

test() ->
    Data = <<"467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..\n">>,
    {Nums, Chars, Len} = walk(Data, 0, [], [], []),
    Offset = trunc(math:sqrt(Len)),
    Reach_Chars = build_char_reach(Chars, Offset, []),
    erlang:display({part1, lists:sum([Num || {Num, Pos} <- Nums, is_adjacent(Reach_Chars, Pos)])}),
    Stars = [ {Char, Reach} || {Char, Reach} <- Reach_Chars, Char == star],
    erlang:display({part1,count_adjacent(Stars, Nums, 0)}).

part1(FileName) ->
    Data = readlines(FileName),
    {Nums, Chars, Len} = walk(Data, 0, [], [], []),
    Offset = trunc(math:sqrt(Len)),
    Reach_Chars = build_char_reach(Chars, Offset, []),
    lists:sum([Num || {Num, Pos} <- Nums, {true, Num} == is_adjacent(Reach_Chars, Pos)]).

part2(FileName) ->
    Data = readlines(FileName),
    {Nums, Chars, Len} = walk(Data, 0, [], [], []),
    Offset = trunc(math:sqrt(Len)),
    Reach_Chars = build_char_reach(Chars, Offset, []),
    Stars = [ {Char, Reach} || {Char, Reach} <- Reach_Chars, Char == star],
    count_adjacent(Stars, Nums, 0).

readlines(FileName) ->
    {ok, Data} = file:read_file(FileName),
    Data.

walk(<<>>, Pos, _, Nums, Chars) ->
    {lists:flatten(Nums), lists:flatten(Chars), Pos};

walk(<<H,T/binary>>, Pos, Num_Acc, Nums, Chars) when H == 10, Num_Acc == [] -> % <<"\n">> would not work
    % erlang:display({newline, Pos}),
    walk(T, Pos, [], Nums, Chars);

walk(<<H,T/binary>>, Pos, Num_Acc, Nums, Chars) when H == 10 -> % <<"\n">> would not work
    % erlang:display({newline, Pos}),
    Num = build_num(Num_Acc),
    walk(T, Pos, [], [Num|[Nums]], Chars);

walk(<<H,T/binary>>, Pos, Num_Acc, Nums, Chars) when H >= 48, H =< 57 ->
    walk(T, Pos+1, [{H-48,Pos}|[Num_Acc]], Nums, Chars);

walk(<<H,T/binary>>, Pos, Num_Acc, Nums, Chars) when H == $#, Num_Acc == [] ->
    walk(T, Pos+1, [], Nums, [{hash,Pos}|[Chars]]);

walk(<<H,T/binary>>, Pos, Num_Acc, Nums, Chars) when H == $# ->
    Num = build_num(Num_Acc),
    walk(T, Pos+1, [], [Num|[Nums]], [{hash,Pos}|[Chars]]);

walk(<<H,T/binary>>, Pos, Num_Acc, Nums, Chars) when H == $&, Num_Acc == [] ->
    walk(T, Pos+1, [], Nums, [{amper,Pos}|[Chars]]);

walk(<<H,T/binary>>, Pos, Num_Acc, Nums, Chars) when H == $& ->
    Num = build_num(Num_Acc),
    walk(T, Pos+1, [], [Num|[Nums]], [{amper,Pos}|[Chars]]);

walk(<<H,T/binary>>, Pos, Num_Acc, Nums, Chars) when H == $+, Num_Acc == [] ->
    walk(T, Pos+1, [], Nums, [{plus,Pos}|[Chars]]);

walk(<<H,T/binary>>, Pos, Num_Acc, Nums, Chars) when H == $+ ->
    Num = build_num(Num_Acc),
    walk(T, Pos+1, [], [Num|[Nums]], [{plus,Pos}|[Chars]]);

walk(<<H,T/binary>>, Pos, Num_Acc, Nums, Chars) when H == $%, Num_Acc == [] ->
    walk(T, Pos+1, [], Nums, [{pct,Pos}|[Chars]]);

walk(<<H,T/binary>>, Pos, Num_Acc, Nums, Chars) when H == $% ->
    Num = build_num(Num_Acc),
    walk(T, Pos+1, [], [Num|[Nums]], [{pct,Pos}|[Chars]]);

walk(<<H,T/binary>>, Pos, Num_Acc, Nums, Chars) when H == $$, Num_Acc == [] ->
    walk(T, Pos+1, [], Nums, [{dollar,Pos}|[Chars]]);

walk(<<H,T/binary>>, Pos, Num_Acc, Nums, Chars) when H == $$ ->
    Num = build_num(Num_Acc),
    walk(T, Pos+1, [], [Num|[Nums]], [{dollar,Pos}|[Chars]]);

walk(<<H,T/binary>>, Pos, Num_Acc, Nums, Chars) when H == $*, Num_Acc == [] ->
    walk(T, Pos+1, [], Nums, [{star,Pos}|[Chars]]);

walk(<<H,T/binary>>, Pos, Num_Acc, Nums, Chars) when H == $* ->
    Num = build_num(Num_Acc),
    walk(T, Pos+1, [], [Num|[Nums]], [{star,Pos}|[Chars]]);
    
walk(<<H,T/binary>>, Pos, Num_Acc, Nums, Chars) when H == $/, Num_Acc == [] ->
    walk(T, Pos+1, [], Nums, [{slash,Pos}|[Chars]]);

walk(<<H,T/binary>>, Pos, Num_Acc, Nums, Chars) when H == $/ ->
    Num = build_num(Num_Acc),
    walk(T, Pos+1, [], [Num|[Nums]], [{slash,Pos}|[Chars]]);

walk(<<H,T/binary>>, Pos, Num_Acc, Nums, Chars) when H == $-, Num_Acc == [] ->
    walk(T, Pos+1, [], Nums, [{dash,Pos}|[Chars]]);

walk(<<H,T/binary>>, Pos, Num_Acc, Nums, Chars) when H == $- ->
    Num = build_num(Num_Acc),
    walk(T, Pos+1, [], [Num|[Nums]], [{dash,Pos}|[Chars]]);

walk(<<H,T/binary>>, Pos, Num_Acc, Nums, Chars) when H == $=, Num_Acc == [] ->
    walk(T, Pos+1, [], Nums, [{eq,Pos}|[Chars]]);

walk(<<H,T/binary>>, Pos, Num_Acc, Nums, Chars) when H == $= ->
    Num = build_num(Num_Acc),
    walk(T, Pos+1, [], [Num|[Nums]], [{eq,Pos}|[Chars]]);

walk(<<H,T/binary>>, Pos, Num_Acc, Nums, Chars) when H == $@ , Num_Acc == [] ->
    walk(T, Pos+1, [], Nums, [{at,Pos}|[Chars]]);

walk(<<H,T/binary>>, Pos, Num_Acc, Nums, Chars) when H == $@ ->
    Num = build_num(Num_Acc),
    walk(T, Pos+1, [], [Num|[Nums]], [{at,Pos}|[Chars]]);

walk(<<_,T/binary>>, Pos, Num_Acc, Nums, Chars) when Num_Acc == [] ->
    walk(T, Pos+1, [], Nums, Chars);

walk(<<_,T/binary>>, Pos, Num_Acc, Nums, Chars) ->
    Num = build_num(Num_Acc),
    walk(T, Pos+1, [], [Num|[Nums]], Chars).
    


build_num(Num_Acc) ->
    Flattened = lists:flatten(Num_Acc),
    % erlang:display({flattened, Flattened}),
    case length(Flattened) of
        1 -> 
            [{C1, P1}] = Flattened,
            Num = C1,
            {Num, [P1]};
        2 ->
            [{C1, P1}, {C2, P2}] = Flattened,
            Num = C1 + 10*C2,
            {Num, [P1, P2]};
        3 ->
            [{C1, P1}, {C2, P2}, {C3, P3}] = Flattened,
            Num = C1 + 10*C2 + 100*C3,
            {Num, [P1, P2, P3]};
        true -> erlang:display({accumulator_error, Num_Acc})
    end.

build_char_reach([], _, Char_Set) ->
    lists:flatten(Char_Set);

build_char_reach([H|T], Offset, Char_Set) ->
    {Char, Pos} = H,
    Dirs = [{-1,-1},{-1,0},{-1,1},{0,-1},{0,0},{0,1},{1,-1},{1,0},{1,1}],
    Reach = [ Pos + (R*Offset) + C || {R,C} <- Dirs],
    build_char_reach(T, Offset, [{Char, Reach}|[Char_Set]]).

is_adjacent([], _) ->
    false;

is_adjacent([H|T], Pos) ->
    {_, Reach} = H,
    % erlang:display({num, Num, pos,Pos, char, Char, reach,Reach}),
    Intersect =  ([] /= [X || X <- Pos, Y <- Reach, X == Y]),
    % erlang:display({inter, Intersect}),
    case Intersect of
        true -> true;
        false -> is_adjacent(T, Pos)
    end.

is_adj_p2([], _, Adj_List) ->
    Flattened = lists:flatten(Adj_List),
    if
        length(Flattened) == 2 -> lists:nth(1, Flattened) * lists:nth(2, Flattened);
        true -> 0
    end;

is_adj_p2([H|T], Reach, Adj_List) ->
    {Num, Pos} = H,
    Intersect =  ([] /= [X || X <- Pos, Y <- Reach, X == Y]),
    case Intersect of
        true -> 
            is_adj_p2(T, Reach, [Num|[Adj_List]]);
        false -> is_adj_p2(T, Reach, Adj_List)
    end.

count_adjacent([], _, Num_Acc) ->
    Num_Acc;

count_adjacent([H|T], Nums, Num_Acc) ->
    {_, Reach} = H,
    Num = is_adj_p2(Nums, Reach, []),
    count_adjacent(T, Nums, Num_Acc + Num).