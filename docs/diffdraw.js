var coord_to_pos = function(start, i, j) {
    var h_spacing = 60;
    var v_spacing = 60;
    return { x: start.x + h_spacing*j,
             y: start.y + v_spacing*i
    };
};

var draw_line = function(s, from_x, from_y, to_x, to_y) {
    var p = s.line(from_x, from_y, to_x, to_y);
    p.attr({
        fill: "#000"
       ,stroke: "#000"
       ,strokeWidth: 2
    });
    return p;
}

var draw_grid_dots = function(s, start, rows, cols) {
    for (var i = 0; i < rows; i++) {
        for (var j = 0; j < cols; j++) {
            var pos = coord_to_pos(start, i, j);
            var bigCircle = s.circle(pos.x, pos.y, 5);
            bigCircle.attr({
                fill: black
               ,stroke: black
               ,strokeWidth: 3
            });
        }
    }
};


var draw_diag_line = function(s, start, coord1, coord2) {
    var pos1 = coord_to_pos(start, coord1.x, coord1.y);
    var pos2 = coord_to_pos(start, coord2.x, coord2.y);
    return draw_line(s, pos1.x, pos1.y, pos2.x, pos2.y);
}

// returns an object with two members, 'down' and 'right', both of which
// are 2D arrays of lines, where down[i][j] is the downward line *starting*
// at node (i, j) (and ditto for 'right'
var draw_grid_lines = function(s, start, rows, cols) {
    var lines = [];
    for (var i = 0; i < rows; i++) {
        lines.push([]);
        for (var j = 0; j < cols; j++) {
            var pos = coord_to_pos(start, i, j);

            // draw the downward lines
            if (i !== rows - 1) {
                var posd = coord_to_pos(start, i+1, j);
                lines[i].push( draw_line(s, pos.x, pos.y, posd.x, posd.y) );
            }

            // draw the rightward line
            if (j !== cols - 1) {
                var posr = coord_to_pos(start, i, j+1);
                lines[i].push( draw_line(s, pos.x, pos.y, posr.x, posr.y) );
            }
        }
    }
    return lines;
};
