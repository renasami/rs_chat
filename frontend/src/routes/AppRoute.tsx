import React, { JSX, useContext } from "react";
import { Navigate, Route, Routes } from "react-router-dom";
import { MainRoutes } from "./MainRoutes";
import { ErrorRoutes } from "./ErrorRoutes";
import { AuthRoutes } from "./AuthRouts";
import { AuthContext } from "../layout/AuthProvider";

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
