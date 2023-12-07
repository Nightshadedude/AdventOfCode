-module(day3).
-export([part1/1, part2/1,test/0]).

test() ->
    L = <<"467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..">>,
    {Blob, Matches, Size} = bin_match(L),
    %erlang:display({Blob, Matches, Size}),
    Found = look_around(Blob, Matches, Size, []),
    Nums = extract_numbers(Blob, 0, [], []),
    Adj = [Num || {Num, PL} <- Nums, length(Found)>length(lists:subtract(Found,PL))],
    lists:sum(Adj).

part1(FileName) ->
    L = readlines(FileName),
    {Blob, Matches, Size} = bin_match(L),
    %erlang:display({Blob, Matches, Size}),
    Found = look_around(Blob, Matches, Size, []),
    % Nums = extract_numbers(Blob, 0, [], []),
    % Adj = [Num || {Num, PL} <- Nums, length(Found)>length(lists:subtract(Found,PL))],
    % lists:sum(Adj).
    Found.

part2(FileName) ->
    L = readlines(FileName),
    L.

readlines(FileName) ->
    {ok, Data} = file:read_file(FileName),
    Data.

bin_match(Data) ->
    Blob = binary:replace(Data, <<"\n">>, <<"">>, [global]),
    {Blob, binary:matches(Blob, [<<"#">>,<<"&">>,<<"+">>,<<"%">>,<<"$">>,<<"*">>,<<"/">>,<<"-">>,<<"=">>,<<"@">>], []), trunc(math:sqrt(byte_size(Blob)))}.


look_around(_, [], _, Found) ->
    erlang:display({found, Found}),
    lists:uniq(lists:flatten(Found));

look_around(Blob, [H|T], Size, Found) ->
    {Loc, _} = H,
    % No symbols on edges, so no need to check % (-1,-1),(-1,0),(-1,1) % (0, -1),( Loc),( 0,1) % (1, -1),( 1,0),( 1,1)
    Dirs = [{-1,-1},{-1,0},{-1,1},{0,-1},{0,1},{1,-1},{1,0},{1,1}],
    % erlang:display({loc, Loc, dirs, Dirs}),
    New_Found = lists:flatten(check_for_num(Dirs, Blob, Loc, Size, Found)),
    look_around(Blob, T, Size, [New_Found|[Found]]).

check_for_num([], _, _, _, Found) ->
    Found;

check_for_num([H|T], Blob, Loc, Size, Found) ->
    % erlang:display(H),
    {Row_Offset, Col_Offset} = H,
    Check_Loc = Loc + (Row_Offset * Size) + Col_Offset,
    Char = binary:at(Blob, Check_Loc),
    if
        Char > 47,Char < 58 -> 
            % erlang:display({"Found number at", Check_Loc, "Char", Char}),
            check_for_num(T, Blob, Loc, Size, [Check_Loc|[Found]]);
        true -> check_for_num(T, Blob, Loc, Size, Found)
    end.

extract_numbers(Blob, Pos, _, Nums) when Pos >= byte_size(Blob) ->
    erlang:display({extract, lists:flatten(Nums)}),
    lists:flatten(Nums);

extract_numbers(Blob, Pos, Acc, Nums) when Pos < byte_size(Blob) ->
    % erlang:display({extract, acc, lists:flatten(Acc), length(lists:flatten(Acc))}),
    Char = binary:at(Blob, Pos),
    if
        Char > 47,Char < 58 -> extract_numbers(Blob, Pos+1, [{Char,Pos}|[Acc]], Nums);
        true ->
            Flattened = lists:flatten(Acc),
            case length(Flattened) of
                0 ->
                    extract_numbers(Blob, Pos+1, Acc, Nums);
                1 -> 
                    [{C1, P1}] = Flattened,
                    Num = C1-48,
                    extract_numbers(Blob, Pos+1, [], [{Num, [P1]}|[Nums]]);
                2 ->
                    [{C1, P1}, {C2, P2}] = Flattened,
                    Num = C1-48 + 10*(C2-48),
                    extract_numbers(Blob, Pos+1, [], [{Num, [P1, P2]}|[Nums]]);
                3 ->
                    [{C1, P1}, {C2, P2}, {C3, P3}] = Flattened,
                    Num = C1-48 + 10*(C2-48) + 100*(C3-48),
                    extract_numbers(Blob, Pos+1, [], [{Num, [P1, P2, P3]}|[Nums]]);
                true -> erlang:display({accumulator_error, Acc})
            end
    end.

%% char list [<<"#">>,<<"&">>,<<"+">>,<<"%">>,<<"$">>,<<"*">>,<<"/">>,<<"-">>,<<"=">>,<<"@">>]
% binary:matches(Blob, [<<"#">>,<<"&">>,<<"+">>,<<"%">>,<<"$">>,<<"*">>,<<"/">>,<<"-">>,<<"=">>,<<"@">>], [])



