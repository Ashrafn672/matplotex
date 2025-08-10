defmodule Matplotex.Nif do
  use Rustler, otp_app: :matplotex, crate: :matplotex_areal

  def compute_statistics(_numbers), do: raise "compute_statistics/1 not implemented"

  def statistics(numbers) when is_list(numbers) do
    case compute_statistics(numbers) do
      {:ok, min, max, intervals} ->
        {:ok, %Matplotex.Nif.Statistics{min: min, max: max, intervales: intervals}}
      {:error, reason} ->
        {:error, reason}
      _ -> {:unknown, :unknown}
    end
  end
end
