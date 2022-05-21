const path = require('path');

module.exports = {
	mode: "development",
	entry: {
		main: ["@babel/polyfill", "./src/index.js"],
	},
	output: {
		path: path.resolve(__dirname, "build"),
	},
	devServer: {
		contentBase: path.join(__dirname, "build"),
		compress: false,
		port: 9000,
	},
	module: {
		rules: [
			{
				test: /\.jsx?$/,
				exclude: [/node_modules/],
				use: [
					{
						loader: 'babel-loader',
					},
				],
			},
			{
				test: /\.s[ac]ss$/i,
				use: [
					'style-loader',
					'css-loader',
					'sass-loader',
				],
			},
		],
	},
};
