// the 's' parameter seen throughout is the Snap.svg object


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
       ,strokeWidth: 1
    });
    return p;
}

var draw_grid_dots = function(s, start, rows, cols) {
    var black = "#222";
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
    var lines = {down: [], right: []};
    for (var i = 0; i < rows; i++) {
        lines.down.push([]);
        lines.right.push([]);
        for (var j = 0; j < cols; j++) {
            var pos = coord_to_pos(start, i, j);

            // draw the downward lines
            if (i !== rows - 1) {
                var posd = coord_to_pos(start, i+1, j);
                lines.down[i].push( draw_line(s, pos.x, pos.y, posd.x, posd.y) );
            }

            // draw the rightward line
            if (j !== cols - 1) {
                var posr = coord_to_pos(start, i, j+1);
                lines.right[i].push( draw_line(s, pos.x, pos.y, posr.x, posr.y) );
            }
        }
    }
    return lines;
};

// this whole dumb thing is for recursively generated nested callbacks
// in order to sequence Snap.svg animations. pass in a sequence of Snap.svg
// elements.
//
// also the animation logic is currently hard-coded. to make this fully generic,
// need to pass in an animation function of some sort. but i only need this!
var animation_sequencer = function(seq) {
    var delay = 150;
    var fill_stroke = {fill: "#ff6d1f", stroke: "#ff2d1f", strokeWidth: 5};

    var make_non_terminal_callback = function(n) {
        return function() {
            seq[n].animate(fill_stroke, delay, null,
                                  make_callback(n+1));
        };
    };

    var make_terminal_callback = function(n) {
        return function() {
            seq[n].animate(fill_stroke, delay);
        };
    };

    var make_callback = function(n) {
        if(n === seq.length - 1) {
            return make_terminal_callback(n);
        } else {
            return make_non_terminal_callback(n);
        }
    };

    return make_callback(0);
};
