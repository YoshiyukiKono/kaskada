## {{ function.name }}

{{ function.signature  | link_fenl_types }}

{{ function.short_doc }}

{%- if function.experimental %}

{{ function.experimental | warning_block | trim }}
{%- endif %}
{%- if function.long_doc %}

{{ function.long_doc -}}
{%- endif %}
**Tags:**
{%- for tag in function.tags | sort %} [{{ tag }}](#{{ tag }}-functions){%- endfor -%}
{%- if function.operator %} [operator](#operators){%- endif -%}

{%- if function.examples -%}
{%- for example in function.examples %}

{% include "partials/example.md" %}
{%- endfor -%}
{%- endif %}
