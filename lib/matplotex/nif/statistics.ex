defmodule Matplotex.Nif.Statistics do
  defstruct min: 0, max: 0, intervales: []

  @type t :: %__MODULE__{
          min: number(),
          max: number(),
          intervales: list(number())
        }


end
