import React from "react";
import { Route, Routes } from "react-router-dom";
import { MainRoutes } from "./MainRoutes";
import { ErrorRoutes } from "./ErrorRoutes";
import { AuthRoutes } from "./AuthRouts";

const AppRoutes: React.FC = () => {
  return (
    <Routes>
      {AuthRoutes()}
      <Route path="/">
        {MainRoutes()}
        {ErrorRoutes()}
      </Route>
    </Routes>
  );
};

export default AppRoutes;
