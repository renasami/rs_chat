import { useQuery } from "react-query";
import { api } from "../api/api";

export const useChat = () => {
  // getRooms

  const result = useQuery<any>("rooms", api.getRooms, {
    retry: false,
  });

  return {
    rooms: result.data == undefined || null ? null : result.data,
    isLoading: result.isLoading,
  };
};
