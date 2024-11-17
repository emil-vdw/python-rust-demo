from enum import Enum
from typing import List
from typing_extensions import Self

class Supplier(Enum):
    Edeka: Self
    SystemU: Self
    GS1: Self

class Ingredient:
    name: str
    def __init__(self, name: str) -> None: ...

class Product:
    id: str  # UUID
    supplier: Supplier
    gtin: str
    description: str
    ingredients: List[Ingredient]
    
    def __init__(
        self,
        supplier: Supplier,
        gtin: str,
        description: str,
        ingredients: List[Ingredient]
    ) -> None: ...

class ProductRef:
    product: Product
    
    def __init__(
        self,
        supplier: Supplier,
        gtin: str,
        description: str,
        ingredients: List[Ingredient]
    ) -> None: ...
    
    @property
    def inner(self) -> Product: ...
