from ai_python.src.parser import parse_board_to_list, parse_all_board

def test_parse_rows_parsing():
    string = "scou|scou|colo|lieu|mine|serg|flag|capt|mine|scou|scou|bomb|scou|lieu|majo|bomb|serg|majo|majo|spy|Gene|mine|Mars|capt|lieu|bomb|bomb|capt|mine|scou|capt|serg|lieu|colo|bomb|mine|scou|serg|scou|bomb"
    res = parse_board_to_list(string, 'R') 
    print(res)
    assert len(res) == 4
    for row in res:
        assert len(row) == 10
        
def test_parse_board():
    blue = "scou|scou|colo|lieu|mine|serg|flag|capt|mine|scou|scou|bomb|scou|lieu|majo|bomb|serg|majo|majo|spy|Gene|mine|Mars|capt|lieu|bomb|bomb|capt|mine|scou|capt|serg|lieu|colo|bomb|mine|scou|serg|scou|bomb"
    red = "scou|scou|colo|lieu|mine|serg|flag|capt|mine|scou|scou|bomb|scou|lieu|majo|bomb|serg|majo|majo|spy|Gene|mine|Mars|capt|lieu|bomb|bomb|capt|mine|scou|capt|serg|lieu|colo|bomb|mine|scou|serg|scou|bomb"
    board = parse_all_board(red, blue)
    sta = board.state()
    assert len(sta) == 10
    for row in sta:
        assert len(row) == 10
