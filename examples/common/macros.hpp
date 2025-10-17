{%- macro cxx_type(type) -%}
{%- if type.type == TYPES.Void -%}
void
{%- elif type.type == TYPES.Bool -%}
bool
{%- elif type.type == TYPES.Char -%}
char
{%- elif type.type == TYPES.U8 -%}
uint8_t
{%- elif type.type == TYPES.I8 -%}
int8_t
{%- elif type.type == TYPES.U16 -%}
uint16_t
{%- elif type.type == TYPES.I16 -%}
int16_t
{%- elif type.type == TYPES.U32 -%}
uint32_t
{%- elif type.type == TYPES.I32 -%}
int32_t
{%- elif type.type == TYPES.U64 -%}
uint64_t
{%- elif type.type == TYPES.I64 -%}
int64_t
{%- elif type.type == TYPES.F32 -%}
float
{%- elif type.type == TYPES.F64 -%}
double
{%- elif type.type == TYPES.Name -%}
{{ type.name }}
{%- elif type.type == TYPES.PtrMut -%}
{{ self::cxx_type(type = type.inner) }}*
{%- elif type.type == TYPES.PtrConst -%}
{{ self::cxx_type(type = type.inner) }} const*
{%- elif type.type == TYPES.ConstRef -%}
{{ self::cxx_type(type = type.inner) }} const&
{%- else -%}
{{ throw(message = "Unsupported type") }}
{%- endif -%}
{%- endmacro -%}

{%- macro cxx_arguments_with_names(args) -%}
{%- for arg in args -%}
{% if arg.constant %}const {% endif %}{{ self::cxx_type(type = arg.type) }} {{ arg.name }}{% if not loop.last %}, {% endif %}
{%- endfor -%}
{%- endmacro -%}

{%- macro cxx_argument_types(args) -%}
{%- for arg in args -%}
{% if arg.constant %}const {% endif %}{{ self::cxx_type(type = arg.type) }}{% if not loop.last %}, {% endif %}
{%- endfor -%}
{%- endmacro -%}
