-module(day6).
-export([part1/1, part2/1, make_2d_arr/2, get_element/3, set_element/4, print_arr/1]).

part1(FileName) -> 
    L = parse(readlines(FileName)),
    Arr = make_2d_arr(1000,1000),
    Final = flipper(L, Arr),
    Final.

part2(FileName) ->
    L = parse(readlines(FileName)),
    Arr = make_2d_arr(1000,1000),
    Final = flipper_2(L, Arr),
    Final.
    
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

parse(String) ->
    string:tokens(String, "\n").

flipper([], Arr) ->
    lists:sum(lists:flatten([array:to_list(R) || R <- array:to_list(Arr)]));
flipper([LH|LT], Arr) -> 
    erlang:display(LH),
    [SH|ST] = string:split(LH, " "),
    if
        SH == "turn" ->
            [TH|TT] = string:split(ST, " "),
            if 
                TH == "on" -> flipper(LT, toggle(on, tail_to_coord(TT), Arr));
                TH == "off" -> flipper(LT, toggle(off, tail_to_coord(TT), Arr))
            end;
        SH == "toggle" -> flipper(LT, toggle(toggle, tail_to_coord(ST), Arr));
        true -> exit
    end.

toggle(on, {LX,LY,LY_Base,RX,RY}, Arr) ->
    New_Arr = set_element(LX, LY, Arr, 1),
    if 
        (LY == RY) and (LX < RX) -> toggle(on, {LX+1,LY_Base,LY_Base,RX,RY}, New_Arr);
        LY < RY -> toggle(on, {LX,LY+1,LY_Base,RX,RY}, New_Arr);
        LX < RX -> toggle(on, {LX+1,LY,LY_Base,RX,RY}, New_Arr);
        true -> New_Arr
    end;

toggle(off, {LX,LY,LY_Base,RX,RY}, Arr) ->
    New_Arr = set_element(LX, LY, Arr, 0),
    if 
        (LY == RY) and (LX < RX) -> toggle(off, {LX+1,LY_Base,LY_Base,RX,RY}, New_Arr);
        LY < RY -> toggle(off, {LX,LY+1,LY_Base,RX,RY}, New_Arr);
        LX < RX -> toggle(off, {LX+1,LY,LY_Base,RX,RY}, New_Arr);
        true -> New_Arr
    end;

toggle(toggle, {LX,LY,LY_Base,RX,RY}, Arr) ->
    Element = get_element(LX, LY, Arr),
    case Element of 
        0 -> 
            New_Arr = set_element(LX, LY, Arr, 1);
        1 -> 
            New_Arr = set_element(LX, LY, Arr, 0);
        true ->
            erlang:display(Element),
            New_Arr = Arr
    end,
    if 
        (LY == RY) and (LX < RX) -> toggle(toggle, {LX+1,LY_Base,LY_Base,RX,RY}, New_Arr);
        LY < RY -> toggle(toggle, {LX,LY+1,LY_Base,RX,RY}, New_Arr);
        LX < RX -> toggle(toggle, {LX+1,LY,LY_Base,RX,RY}, New_Arr);
        true -> New_Arr
    end;
toggle(_, _, Arr) ->
    erlang:display("dafuq?"),
    Arr.





flipper_2([], Arr) ->
    lists:sum(lists:flatten([array:to_list(R) || R <- array:to_list(Arr)]));
flipper_2([LH|LT], Arr) -> 
    erlang:display(LH),
    [SH|ST] = string:split(LH, " "),
    if
        SH == "turn" ->
            [TH|TT] = string:split(ST, " "),
            if 
                TH == "on" -> flipper_2(LT, toggle_2(on, tail_to_coord(TT), Arr));
                TH == "off" -> flipper_2(LT, toggle_2(off, tail_to_coord(TT), Arr))
            end;
        SH == "toggle" -> flipper_2(LT, toggle_2(toggle, tail_to_coord(ST), Arr));
        true -> exit
    end.

toggle_2(Toggle_Type, {LX,LY,LY_Base,RX,RY}, Arr) ->
    Element = get_element(LX, LY, Arr),
    case Toggle_Type of
        on -> New_Arr = set_element(LX, LY, Arr, Element + 1);
        toggle -> New_Arr = set_element(LX, LY, Arr, Element + 2);
        off -> New_Arr = set_element(LX, LY, Arr, lists:max([Element - 1,0]))
    end,

    if 
        (LY == RY) and (LX < RX) -> toggle_2(Toggle_Type, {LX+1,LY_Base,LY_Base,RX,RY}, New_Arr);
        LY < RY -> toggle_2(Toggle_Type, {LX,LY+1,LY_Base,RX,RY}, New_Arr);
        LX < RX -> toggle_2(Toggle_Type, {LX+1,LY,LY_Base,RX,RY}, New_Arr);
        true -> New_Arr
    end.

    
tail_to_coord(TT) ->
    [L|R] = string:split(TT, " through "),
    [LX,LY|_] = string:split(L, ","),
    [RX,RY|_] = string:split(R, ","),
    {erlang:list_to_integer(LX),erlang:list_to_integer(LY),erlang:list_to_integer(LY),erlang:list_to_integer(RX),erlang:list_to_integer(RY)}.

make_2d_arr(Row,Col) ->
    Arr_Row = array:new(Row, {default, 0}),
    Arr = array:new(Col, {default, Arr_Row}),
    Arr.

get_element(Row, Col, Arr) ->
    Arr_Row = array:get(Row, Arr),
    array:get(Col, Arr_Row).

set_element(Row, Col, Arr, Element) ->
    Read_Row = array:get(Row, Arr),
    Write_Row = array:set(Col, Element, Read_Row),
    array:set(Row, Write_Row, Arr).



print_arr(Arr) ->
    [array:to_list(R) || R <- array:to_list(Arr)].
