-module(day5).
-export([part1/1, part2/1]).

part1(FileName) ->
    L = parse_line(readlines(FileName)),
    {Seed, StS, StF, FtW, WtL, LtT,TtH, HtL} = parse_file(L,none,[],[],[],[],[],[],[],[]),
    Soil = [mapping(S, StS) || S <- Seed],
    Fert = [mapping(So, StF) || So <- Soil],
    Water = [mapping(F, FtW) || F <- Fert],
    Light = [mapping(W, WtL) || W <- Water],
    Temp = [mapping(Lt, LtT) || Lt <- Light],
    Hum = [mapping(T, TtH) || T <- Temp],
    Loc = [mapping(H, HtL) || H <- Hum],
    lists:min(Loc).

part2(FileName) ->
    L = parse_line(readlines(FileName)),
    {Seed, StS, StF, FtW, WtL, LtT,TtH, HtL} = parse_file(L,none,[],[],[],[],[],[],[],[]),
    P2_Seed = expand(Seed, []),
    erlang:display({seedsbuilt, length(P2_Seed)}).
    % Soil = [mapping(S, StS) || S <- P2_Seed],
    % erlang:display({soilbuilt}),
    % Fert = [mapping(So, StF) || So <- Soil],
    % erlang:display({fertbuilt}),
    % Water = [mapping(F, FtW) || F <- Fert],
    % erlang:display({waterbuilt}),
    % Light = [mapping(W, WtL) || W <- Water],
    % erlang:display({lightbuilt}),
    % Temp = [mapping(Lt, LtT) || Lt <- Light],
    % erlang:display({tempbuilt}),
    % Hum = [mapping(T, TtH) || T <- Temp],
    % erlang:display({humbuilt}),
    % Loc = [mapping(H, HtL) || H <- Hum],
    % lists:min(Loc).

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

parse_file([], _, Seed, StS, StF, FtW, WtL, LtT, TtH, HtL) ->
    {Seed, lists:reverse(StS), lists:reverse(StF), lists:reverse(FtW), lists:reverse(WtL), lists:reverse(LtT), lists:reverse(TtH), lists:reverse(HtL)};
 
parse_file([H|T], Current_Parse, Seeds, StS, StF, FtW, WtL, LtT, TtH, HtL) ->
    if
        H == "seed-to-soil map:" -> parse_file(T, sts, Seeds, StS, StF, FtW, WtL, LtT, TtH, HtL);
        H == "soil-to-fertilizer map:" -> parse_file(T, stf, Seeds, StS, StF, FtW, WtL, LtT, TtH, HtL);
        H == "fertilizer-to-water map:" -> parse_file(T, ftw, Seeds, StS, StF, FtW, WtL, LtT, TtH, HtL);
        H == "water-to-light map:" -> parse_file(T, wtl, Seeds, StS, StF, FtW, WtL, LtT, TtH, HtL);
        H == "light-to-temperature map:" -> parse_file(T, ltt, Seeds, StS, StF, FtW, WtL, LtT, TtH, HtL);
        H == "temperature-to-humidity map:" -> parse_file(T, tth, Seeds, StS, StF, FtW, WtL, LtT, TtH, HtL);
        H == "humidity-to-location map:" -> parse_file(T, htl, Seeds, StS, StF, FtW, WtL, LtT, TtH, HtL);
        true ->
            case Current_Parse of
                none -> parse_file(T, Current_Parse, parse_seeds(H), StS, StF, FtW, WtL, LtT, TtH, HtL);
                sts -> parse_file(T, Current_Parse, Seeds, [parse_nums(H)|StS], StF, FtW, WtL, LtT, TtH, HtL);
                stf -> parse_file(T, Current_Parse, Seeds, StS, [parse_nums(H)|StF], FtW, WtL, LtT, TtH, HtL);
                ftw -> parse_file(T, Current_Parse, Seeds, StS, StF, [parse_nums(H)|FtW], WtL, LtT, TtH, HtL);
                wtl -> parse_file(T, Current_Parse, Seeds, StS, StF, FtW, [parse_nums(H)|WtL], LtT, TtH, HtL);
                ltt -> parse_file(T, Current_Parse, Seeds, StS, StF, FtW, WtL, [parse_nums(H)|LtT], TtH, HtL);
                tth -> parse_file(T, Current_Parse, Seeds, StS, StF, FtW, WtL, LtT, [parse_nums(H)|TtH], HtL);
                htl -> parse_file(T, Current_Parse, Seeds, StS, StF, FtW, WtL, LtT, TtH, [parse_nums(H)|HtL])
            end
    end.

parse_seeds(L) ->
    [_|T] = string:lexemes(L, " "),
    {I, _}= lists:unzip([string:to_integer(N) || N <- T]),
    I.

parse_nums(L) ->
    Num_Strings = string:lexemes(L, " "),
    {I, _}= lists:unzip([string:to_integer(N) || N <- Num_Strings]),
    I.

mapping(Mapped, []) ->
    Mapped;

mapping(Mapped, [[Dest_Start, Source_Start, Range]|_]) when Mapped >= Source_Start andalso Mapped =< (Source_Start + Range) ->
    % io:write({mapped, Mapped, input, [Dest_Start, Source_Start, Range], offset, Dest_Start - Source_Start, new_num,  Mapped +Dest_Start - Source_Start}),
    % erlang:display(break),
    Offset =  Dest_Start - Source_Start,
    Mapped + Offset;

mapping(Mapped, [_|T]) ->
    mapping(Mapped, T).

expand([], Acc) ->
    Acc;


expand([Start,Range|T], Acc) ->
    erlang:display({buildingrange, Start, Start+Range-1}),
    Acc_Range = lists:seq(Start, (Start+Range-1)),
    expand(T, [Acc_Range]++Acc).
