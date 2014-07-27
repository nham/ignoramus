var canvas = new fabric.Canvas('c');


// start is a dictionary { x: <number>, y: <number> }
// i, j are numbers
// I'm using "matrix coordinates" here. if (i, j) is a coordinate
// pair, i is the row and j is the column
var coord_to_pos = function(start, i, j) {
    var h_spacing = 60;
    var v_spacing = 60;
    return { x: start.x + h_spacing*j,
             y: start.y + v_spacing*i
    };
};

var grid_dots = function(start, rows, cols) {
    var circs = [];

    for (var i = 0; i < rows; i++) {
        for (var j = 0; j < cols; j++) {
            var pos = coord_to_pos(start, i, j);
            circs.push(new fabric.Circle({
                radius: 5, fill: 'black', left: pos.x, top: pos.y,
                originX: "center", originY: "center",
            }));
        }
    }

    for (var i = 0; i < circs.length; i++) {
        canvas.add(circs[i]);
    }
};

var grid_lines = function(start, rows, cols) {
    var lines = [];

    for (var i = 0; i < rows; i++) {
        for (var j = 0; j < cols; j++) {
            var pos = coord_to_pos(start, i, j);

            if (i !== rows - 1) {
                var posr = coord_to_pos(start, i+1, j);
                lines.push(new fabric.Line([pos.x, pos.y, posr.x, posr.y], {
                    fill: 'black', stroke: 'grey', strokeWidth: 2
                }));
            }

            if (j !== cols - 1) {
                var posb = coord_to_pos(start, i, j+1);
                lines.push(new fabric.Line([pos.x, pos.y, posb.x, posb.y], {
                    fill: 'black', stroke: 'grey', strokeWidth: 2
                }));
            }
        }
    }

    for (var i = 0; i < lines.length; i++) {
        canvas.add(lines[i]);
    }
};

var diag_line = function(start, coord1, coord2) {
    var pos1 = coord_to_pos(start, coord1.x, coord1.y);
    var pos2 = coord_to_pos(start, coord2.x, coord2.y);
    canvas.add(new fabric.Line([pos1.x, pos1.y, pos2.x, pos2.y], {
        fill: 'black', stroke: 'grey', strokeWidth: 2
    }));
}

// start is an object {x: <canvas pos>, y: <canvas pos>}
// rows/cols are numbers
var grid = function(start, rows, cols, diags) {
    diags = diags || [];

    var line_start = {x: start.x - 0.5, y: start.y - 0.5};
    grid_lines(line_start, rows, cols);

    for (var i = 0; i < diags.length; i++) {
        diag_line(line_start, {x: diags[i][0], y: diags[i][1]},
                              {x: diags[i][0]+1, y: diags[i][1]+1});
    }

    grid_dots(start, rows, cols);
};

var grid_start = { x: 50, y: 50 };
grid(grid_start, 7, 5, [[1, 2], [3,1]]);
