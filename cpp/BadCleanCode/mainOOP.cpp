#include <iostream>
#include <vector>

using namespace std;

class Shape
{
public:
    virtual double area() const = 0;
};

class Square : public Shape
{
public:
    Square(double side) : mySide(side) {}
    double area() const override
    {
        return mySide * mySide;
    }
private:
    double mySide;
};

class Rectangle : public Shape
{
public:
    Rectangle(double width, double height) : myWidth(width), myHeight(height) {}
    double area() const override
    {
        return myWidth * myHeight;
    }
private:
    double myWidth;
    double myHeight;
};

class Triangle : public Shape
{
public:
    Triangle(double base, double height) : myBase(base), myHeight(height) {}
    double area() const override
    {
        return 0.5 * myBase * myHeight;
    }
private:
    double myBase;
    double myHeight;
};

class Circle : public Shape
{
public:
    Circle(double radius) : myRadius(radius) {}
    double area() const override
    {
        return 3.1415 * myRadius * myRadius;
    }
private:
    double myRadius;
};

double sumShapeArea(const vector<Shape*>& shapes)
{
    double sum = 0.0;
    for (const auto* s : shapes)
    {
        sum += s->area();
    }
    return sum;
}

int main() {

}
