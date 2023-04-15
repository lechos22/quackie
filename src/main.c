#include <ncurses.h>
#include <math.h>
#include <unistd.h>

typedef struct {
    float x;
    float y;
} Point;

typedef struct {
    Point p1;
    Point p2;
    Point p3;
} Triangle;

const Triangle base_triangles[] = {
    // Body
    {{0.3, 0.45},
     {0.3, 0.75},
     {0.13, 0.5}},
    {{0.3, 0.45},
     {0.3, 0.75},
     {0.7, 0.75}},
    {{0.3, 0.45},
     {0.7, 0.45},
     {0.7, 0.75}},
    {{0.7, 0.45},
     {0.7, 0.75},
     {0.8, 0.55}},
    // Head
    {{0.4, 0.45},
     {0.6, 0.45},
     {0.35, 0.3}},
    {{0.45, 0.2},
     {0.6, 0.45},
     {0.35, 0.3}},
    {{0.45, 0.2},
     {0.6, 0.45},
     {0.6, 0.25}},
    {{0.45, 0.2},
     {0.55, 0.2},
     {0.6, 0.25}},
};

const Triangle eye_triangle = {
    {0.525, 0.3},
    {0.5, 0.25},
    {0.475, 0.3}
};

const Triangle beak_triangle = {
    {0.75, 0.365},
    {0.6, 0.45},
    {0.6, 0.3}
};

float signed_triangle_area(Triangle t) {
    return 0.5 * (
            - t.p2.y * t.p3.x
            + t.p1.y * (-t.p2.x + t.p3.x)
            + t.p1.x * (t.p2.y - t.p3.y)
            + t.p2.x * t.p3.y
    );
}

const float EPSILON = 0.0000001;

bool point_inside_triangle(Point pt, Triangle tr) {
    float area = signed_triangle_area(tr);
    float s = 1/(2*area) * (tr.p1.y * tr.p3.x - tr.p1.x * tr.p3.y +
                             (tr.p3.y - tr.p1.y) * pt.x +
                             (tr.p1.x - tr.p3.x) * pt.y);
    float t = 1/(2*area) * (tr.p1.x * tr.p2.y - tr.p1.y * tr.p2.x +
                             (tr.p1.y - tr.p2.y) * pt.x +
                             (tr.p2.x - tr.p1.x) * pt.y);
    return s > -EPSILON && t > -EPSILON && (1-s-t) > -EPSILON;
}

Triangle move_triangle(Triangle t, float x, float y) {
    return (Triangle) {
        {t.p1.x + x, t.p1.y + y},
        {t.p2.x + x, t.p2.y + y},
        {t.p3.x + x, t.p3.y + y}
    };
}

Triangle rotate_triangle(Triangle t, float angle) {
    float c = cos(angle);
    float s = sin(angle);
    return (Triangle) {
        {t.p1.x * c - t.p1.y * s, t.p1.x * s + t.p1.y * c},
        {t.p2.x * c - t.p2.y * s, t.p2.x * s + t.p2.y * c},
        {t.p3.x * c - t.p3.y * s, t.p3.x * s + t.p3.y * c}
    };
}

Triangle rotate_triangle_around_point(Triangle t, Point p, float angle) {
    return move_triangle(rotate_triangle(move_triangle(t, -p.x, -p.y), angle), p.x, p.y);
}

void draw(Point size, float angle){
    Point center = {0.5, 0.5};
    for(float i = 0; i < size.x; i++){
        for(float j = 0; j < size.y; j++){
            Point pt = {i / size.x, j / size.y};
            bool inside_base = 0;
            for(size_t k = 0; k < sizeof(base_triangles)/sizeof(Triangle); k++) {
                Triangle tr = rotate_triangle_around_point(base_triangles[k], center, angle);
                inside_base |= point_inside_triangle(pt, tr);
            }
            inside_base &= !point_inside_triangle(pt, rotate_triangle_around_point(eye_triangle, center, angle));
            if(inside_base) {
                attroff(COLOR_PAIR(1));
                attron(COLOR_PAIR(2));
                mvaddch(j, i, '#');
            }
            else if(point_inside_triangle(pt, rotate_triangle_around_point(beak_triangle, center, angle))) {
                attroff(COLOR_PAIR(2));
                attron(COLOR_PAIR(1));
                mvaddch(j, i, '#');
            }
            else {
                mvaddch(j, i, ' ');
            }
        }
    }
    refresh();
}

int main(void) {
    // Initialize ncurses
    initscr();
    if(!has_colors()) return 1;
    start_color();
    init_pair(1, COLOR_RED, COLOR_BLACK);
    init_pair(2, COLOR_YELLOW, COLOR_BLACK);
    attron(A_BOLD);
    float angle = 0;
    // Main loop
    while(true){
        // Get window size
        Point size;
        getmaxyx(stdscr, size.y, size.x);
        // Rotate the bird
        angle += 0.31415926535897932384626433832795 * 0.5;
        draw(size, angle);
        usleep(50000);
    }
    // End ncurses
    endwin();
    return 0;
}

