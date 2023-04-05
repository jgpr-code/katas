#include <vector>

using namespace std;

enum class ShapeType
{
    SQUARE,
    RECTANGLE,
    TRIANGLE,
    CIRCLE,
};

struct Shape
{
    ShapeType type;
    double width;
    double height;
};

double getArea(const Shape& shape)
{
    switch (shape.type)
    {
        case ShapeType::SQUARE:
            return shape.width * shape.width;
        case ShapeType::RECTANGLE:
            return shape.width * shape.height;
        case ShapeType::TRIANGLE:
            return 0.5 * shape.width * shape.height;
        case ShapeType::CIRCLE:
            return 3.1415 * shape.width * shape.width;
        default:
            return 0.0;
    }
}

double sumShapeArea(const vector<Shape>& shapes)
{
    double sum = 0.0;
    for (const auto s : shapes)
    {
        sum += getArea(s);
    }
    return sum;
}

int main()
{

}