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

    for (var i = 0; i < rows; i++) {
        for (var j = 0; j < cols; j++) {
            var pos = coord_to_pos(grid_start, i, j);
            circs.push(new fabric.Circle({
                radius: 3, fill: 'green', left: pos.x, top: pos.y
            }));
        }
    }

    for (var i = 0; i < circs.length; i++) {
        canvas.add(circs[i]);
    }
};

grid_dots(6,4);

