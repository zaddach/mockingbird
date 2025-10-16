{% import "templates/macros.hpp" as macros  %}
// This file is generated. Do not edit.
#pragma once

#include <gmock/gmock.h>
#include "IWin32Api.hpp"

class MockWin32Api : public IWin32Api {
public:
    MockWin32Api();
    virtual ~MockWin32Api();
    {% for function in functions %}
    MOCK_METHOD({{ macros::cxx_type(type = function.return_type) }}, {{ function.name -}}, ({{- macros::cxx_argument_types(args = function.params) -}}), (override));
    {% endfor %}
};