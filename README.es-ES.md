

# Analizador Open CDS
**AST y Analizador de Código Abierto y Reutilizable para SAP CAP CDS**

[![CI](https://github.com/zkud/open-cds-parser/actions/workflows/ci.yml/badge.svg)](https://github.com/zkud/open-cds-parser/actions/workflows/ci.yml) [![codecov](https://codecov.io/gh/zkud/open-cds-parser/branch/main/graph/badge.svg?token=BITDY89WY7)](https://codecov.io/gh/zkud/open-cds-parser) [![Hits-of-Code](https://hitsofcode.com/github/zkud/open-cds-parser?branch=main)](https://hitsofcode.com/github/zkud/open-cds-parser/view?branch=main) 

## Descripción

**!!! Actualmente, el crate se encuentra en una fase beta inestable y  aún en desarrollo, por lo que tenga precaución al utilizarlo en producción o al depender de la interfaz. !!!**

El ```open-cds-parser``` crate fue diseñado como la base para cualquier herramienta para SAP CAP CDS que necesite trabajar con el lenguaje
y hacerlo de manera eficiente. El crate está robustamente inspirado en un proyecto muy famoso, ESTree para JavaScript, que se
encuentra en la base de eslint y otras herramientas similares.

¡Intente crear su propia herramienta práctica para CAP CDS :)

## Uso

Para comenzar, agregue lo siguiente a su ```Cargo.toml```:
```
open_cds_parser="0.0.1"
```

## Características

- Árbol de Sintaxis Abstracta (AST) de CDS completamente editable 
- Soporte para un uso  similar a eventos mediante el patrón Visitor 

## Licencia

[MIT](LICENSE)
