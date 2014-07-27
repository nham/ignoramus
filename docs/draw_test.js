var canvas = new fabric.Canvas('c');


// start is a dictionary { x: <number>, y: <number> }
// i, j are numbers
var coord_to_pos = function(start, i, j) {
    var h_spacing = 60;
    var v_spacing = 60;
    return { x: start.x + h_spacing*i,
             y: start.y + v_spacing*j
    };
};

var grid_dots = function(rows, cols) {
    var circs = [];
    var grid_start = { x: 50, y: 50 };

    for (var i = 0; i < cols; i++) {
        for (var j = 0; j < rows; j++) {
            var pos = coord_to_pos(grid_start, i, j);
            circs.push(new fabric.Circle({
                radius: 8, fill: 'green', left: pos.x, top: pos.y
            }));
        }
    }

    for (var i = 0; i < circs.length; i++) {
        canvas.add(circs[i]);
    }
};

var grid_lines = function(rows, cols) {
    var lines = [];
    var grid_start = { x: 50, y: 50 };

    for (var i = 0; i < cols; i++) {
        for (var j = 0; j < rows; j++) {
            var pos = coord_to_pos(grid_start, i, j);

            if (i !== cols - 1) {
                var posr = coord_to_pos(grid_start, i+1, j);
                lines.push(new fabric.Line([pos.x, pos.y, posr.x, posr.y], {
                    fill: 'green', stroke: 'green', strokeWidth: 1
                }));
            }

            if (j !== rows - 1) {
                var posb = coord_to_pos(grid_start, i, j+1);
                lines.push(new fabric.Line([pos.x, pos.y, posb.x, posb.y], {
                    fill: 'green', stroke: 'green', strokeWidth: 1
                }));
            }
        }
    }

    for (var i = 0; i < lines.length; i++) {
        canvas.add(lines[i]);
    }
};

grid_dots(6,4);
grid_lines(6,4);
