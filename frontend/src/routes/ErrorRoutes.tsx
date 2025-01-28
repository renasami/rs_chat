import { Route } from "react-router-dom";
import NotFound from "../pages/NotFoundPage";

export const ErrorRoutes = () => {
  return <Route path="*" element={<NotFound />} />;
};
