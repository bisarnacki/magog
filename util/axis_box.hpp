/* axis_box.hpp

   Copyright (C) 2012 Risto Saarelma

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

#ifndef UTIL_AXISBOX_HPP
#define UTIL_AXISBOX_HPP

#include "alg.hpp"
#include "vec.hpp"
#include "core.hpp"
#include <algorithm>

/// Axis-aligned variable-dimension box.
template<class T, int N> class Axis_Box {
 public:
  Axis_Box() {}

  Axis_Box(const Vec<T, N>& min, const Vec<T, N>& dim)
      : min_pt(min), dim_vec(dim) {
    ASSERT(all_of(dim_vec, [](T x) { return x >= 0; }));
  }

  Axis_Box(const Vec<T, N>& dim)
      : min_pt(), dim_vec(dim) {
    ASSERT(all_of(dim_vec, [](T x) { return x >= 0; }));
  }

  bool contains(const Vec<T, N>& pos) const {
    return pairwise_all_of(min_pt, pos, [](T a, T b) { return a <= b; }) &&
        pairwise_all_of(pos, max(), [](T a, T b) { return a < b; });
  }

  bool contains(const Axis_Box<T, N>& other) const {
    for (int i = 0; i < N; i++) {
      if (other.min()[i] < min()[i]) return false;
      if (other.max()[i] > max()[i]) return false;
    }
    return true;
  }

  bool intersects(const Axis_Box<T, N>& other) const {
    int no_intersect = N;
    for (int i = 0; i < N; i++) {
      if (!(other.min()[i] >= max()[i] || min()[i] >= other.max()[i]))
        no_intersect--;
    }
    return !no_intersect;
  }

  const Vec<T, N>& min() const { return min_pt; }

  Vec<T, N> max() const { return min_pt + dim_vec; }

  const Vec<T, N>& dim() const { return dim_vec; }

  Axis_Box<T, N> operator+(const Vec<T, N>& offset) const {
    return Axis_Box<T, N>(min_pt + offset, dim_vec);
  }

  T volume() const {
    return std::accumulate(
        dim_vec.begin(), dim_vec.end(), T(1),
        [] (const T& a, const T& b) { return a * b; });
  }

 private:
  Vec<T, N> min_pt;
  Vec<T, N> dim_vec;
};

typedef Axis_Box<int, 2> ARecti;
typedef Axis_Box<float, 2> ARectf;
typedef Axis_Box<double, 2> ARectd;
typedef Axis_Box<int, 3> ACubei;
typedef Axis_Box<float, 3> ACubef;
typedef Axis_Box<double, 3> ACubed;

#endif
